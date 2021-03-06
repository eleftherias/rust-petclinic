use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequest, Path, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use entity::{owner, pet, pet_type, specialty, vet};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use migration::{Migrator, MigratorTrait};
use owner::Entity as Owner;
use pet::Entity as Pet;
use pet_type::Entity as PetType;
use sea_orm::{entity::prelude::*, Database, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use specialty::Entity as Specialty;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use vet::Entity as Vet;

static SECRET: &[u8] = b"2A82803BD110E4E06C94E581C559DFA";

#[tokio::main]
async fn main() {
    let connection = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::fresh(&connection).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(connection).into_make_service())
        .await
        .unwrap();
}

fn app(connection: DatabaseConnection) -> Router {
    Router::new()
        .route("/vets", get(vets_get))
        .route("/owners", get(owners_get))
        .route("/owners/:owner_id/pets/new", post(pet_create))
        .route("/token", post(authorize))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods(Any),
        )
        .layer(ServiceBuilder::new().layer(Extension(connection)))
}

async fn vets_get(
    Extension(ref conn): Extension<DatabaseConnection>,
    _claims: Claims, // Require authentication for this endpoint
) -> impl IntoResponse {
    let vets: Vec<VetDto> = Vet::find()
        .find_with_related(Specialty)
        .all(conn)
        .await
        .expect("Could not fetch vets")
        .into_iter()
        .map(|t| VetDto {
            id: t.0.id,
            first_name: t.0.first_name,
            last_name: t.0.last_name,
            specialties: t
                .1
                .into_iter()
                .map(|s| SpecialtyDto {
                    id: s.id,
                    name: s.name,
                })
                .collect(),
        })
        .collect();

    Json(vets)
}

async fn owners_get(Extension(ref conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    let owners = Owner::find()
        .all(conn)
        .await
        .expect("Could not fetch owners");

    let mut owner_dtos: Vec<dto::Owner> = Vec::new();
    for owner in owners {
        let pets = owner
            .find_related(Pet)
            .all(conn)
            .await
            .expect("Could not fetch pets");
        let mut pet_dtos: Vec<dto::Pet> = Vec::new();
        for pet in pets {
            let kind = pet
                .find_related(PetType)
                .one(conn)
                .await
                .expect("Could not fetch pet type")
                .unwrap();
            let pet_dto = dto::Pet {
                id: pet.id,
                name: pet.name,
                birth_date: pet.birth_date,
                kind: dto::Type {
                    id: kind.id,
                    name: kind.name,
                },
            };
            pet_dtos.push(pet_dto);
        }
        let owner_dto = dto::Owner {
            id: owner.id,
            first_name: owner.first_name,
            last_name: owner.last_name,
            address: owner.address,
            city: owner.city,
            telephone: owner.telephone,
            pets: pet_dtos,
        };
        owner_dtos.push(owner_dto);
    }

    Json(owner_dtos)
}

async fn pet_create(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(owner_id): Path<i32>,
    Json(payload): Json<dto::NewPet>,
) -> impl IntoResponse {
    pet::ActiveModel {
        name: Set(payload.name.to_owned()),
        birth_date: Set(payload.birth_date.to_owned()),
        type_id: Set(payload.kind_id.to_owned()),
        owner_id: Set(Some(owner_id)),
        ..Default::default()
    }
    .save(conn)
    .await
    .expect("Coud not create pet");

    StatusCode::CREATED
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<String, AuthError> {
    // Allow any username/password combination
    if payload.user.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let expiry = 36000;

    let claims = Claims {
        exp: (now + expiry) as usize,
        iat: now as usize,
        iss: "self".to_owned(),
        scope: "read".to_owned(),
        sub: "user".to_owned(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(token)
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    iss: String,
    scope: String,
    sub: String,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthPayload {
    user: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Serialize)]
struct VetDto {
    id: i32,
    first_name: String,
    last_name: String,
    specialties: Vec<SpecialtyDto>,
}

#[derive(Serialize)]
struct SpecialtyDto {
    id: i32,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sea_orm::Database;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn get_owners_returns_owner_list() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/owners")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let owner_list: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(owner_list.as_array().unwrap().len(), 10);

        let first_owner = owner_list.get(0);
        assert!(first_owner.is_some());
        assert!(first_owner.unwrap().get("id").is_some());
    }

    #[tokio::test]
    async fn post_new_pet_creates_pet() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        // there is likely a better alternative to cloning
        let connection1 = connection.clone();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let new_pet = dto::NewPet {
            name: "Cat".to_owned(),
            birth_date: Date::from_ymd(2015, 3, 15),
            kind_id: 1,
        };

        let new_pet_string = serde_json::to_string(&new_pet).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/owners/1/pets/new")
                    .body(Body::from(new_pet_string))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let new_pet_db: Vec<pet::Model> = Pet::find()
            .filter(pet::Column::Name.contains("Cat"))
            .all(&connection1)
            .await
            .unwrap();

        assert_eq!(new_pet_db.len(), 1);
        assert_eq!(new_pet_db.first().unwrap().owner_id, Some(1));
    }

    #[tokio::test]
    async fn post_token_with_credentials_returns_token() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let user = AuthPayload {
            user: "user".to_owned(),
            password: "password".to_owned(),
        };

        let user_string = serde_json::to_string(&user).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/token")
                    .body(Body::from(user_string))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert!(!body.is_empty());
    }

    #[tokio::test]
    async fn post_token_without_credentials_returns_400() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn get_vets_with_token_returns_vet_list() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let claims = Claims {
            exp: 2000000000, // May 2033
            iat: 1657793083,
            iss: "self".to_owned(),
            scope: "read".to_owned(),
            sub: "user".to_owned(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .header("Authorization", format!("Bearer {token}"))
                    .uri("/vets")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let vet_list: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(vet_list.as_array().unwrap().len(), 2);

        let first_vet = vet_list.get(0);
        assert!(first_vet.is_some());
        assert!(first_vet.unwrap().get("id").is_some());
    }

    #[tokio::test]
    async fn get_vets_with_expired_token_returns_200() {
        let connection = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::fresh(&connection).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        let app = app(connection);

        let claims = Claims {
            exp: 1654041600, // June 2022
            iat: 1657793083,
            iss: "self".to_owned(),
            scope: "read".to_owned(),
            sub: "user".to_owned(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .header("Authorization", format!("Bearer {token}"))
                    .uri("/vets")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
