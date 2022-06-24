use axum::{
    extract::Path,
    http::HeaderValue,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use entity::{owner, pet, pet_type, specialty, vet};
use hyper::{Method, StatusCode};
use migration::{Migrator, MigratorTrait};
use owner::Entity as Owner;
use pet::Entity as Pet;
use pet_type::Entity as PetType;
use sea_orm::{entity::prelude::*, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use specialty::Entity as Specialty;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use vet::Entity as Vet;

#[tokio::main]
async fn main() {
    let connection =
        sea_orm::Database::connect("sqlite::memory:")
            .await
            .unwrap();
    Migrator::fresh(&connection).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    let app = Router::new()
        .route("/vets", get(vets_get))
        .route("/owners", get(owners_get))
        .route("/owners/:owner_id/pets/new", post(pet_create))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .layer(ServiceBuilder::new().layer(Extension(connection)));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn vets_get(Extension(ref conn): Extension<DatabaseConnection>) -> impl IntoResponse {
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

    let mut owner_dtos: Vec<OwnerDto> = Vec::new();
    for owner in owners {
        let pets = owner
            .find_related(Pet)
            .all(conn)
            .await
            .expect("Could not fetch pets");
        let mut pet_dtos: Vec<PetDto> = Vec::new();
        for pet in pets {
            let kind = pet
                .find_related(PetType)
                .one(conn)
                .await
                .expect("Could not fetch pet type")
                .unwrap();
            let pet_dto = PetDto {
                id: pet.id,
                name: pet.name,
                birth_date: pet.birth_date,
                kind: TypeDto {
                    id: kind.id,
                    name: kind.name,
                },
            };
            pet_dtos.push(pet_dto);
        }
        let owner_dto = OwnerDto {
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
    Json(payload): Json<CreatePet>,
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

#[derive(Serialize)]
struct OwnerDto {
    id: i32,
    first_name: String,
    last_name: String,
    address: String,
    city: String,
    telephone: String,
    pets: Vec<PetDto>,
}

#[derive(Serialize)]
struct PetDto {
    id: i32,
    name: String,
    birth_date: Date,
    kind: TypeDto, // type is a keyword in Rust
}

#[derive(Deserialize)]
struct CreatePet {
    name: String,
    birth_date: Date,
    kind_id: i32,
}

#[derive(Serialize)]
struct TypeDto {
    id: i32,
    name: String,
}

// Temporarily disable
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{Request, StatusCode},
//     };
//     use serde_json::Value;
//     use tower::ServiceExt; // for `app.oneshot()`

//     #[tokio::test]
//     async fn vets_get_returns() {
//         let app = app();

//         // `Router` implements `tower::Service<Request<Body>>` so we can
//         // call it like any tower service, no need to run an HTTP server.
//         let response = app
//             .oneshot(Request::builder().uri("/vets").body(Body::empty()).unwrap())
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: Value = serde_json::from_slice(&body).unwrap();
//         let first_vet = body.get(0);
//         assert!(first_vet.is_some());
//         assert!(first_vet.unwrap().get("id").is_some());
//     }
// }
