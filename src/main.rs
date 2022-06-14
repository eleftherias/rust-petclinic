use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use entity::{specialty, vet};
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use specialty::Entity as Specialty;
use tower::ServiceBuilder;
use vet::Entity as Vet;

#[tokio::main]
async fn main() {
    let connection =
        sea_orm::Database::connect("postgres://petclinic:petclinic@localhost/petclinic")
            .await
            .unwrap();
    Migrator::fresh(&connection).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();

    let app = Router::new()
        .route("/vets", get(vets_get))
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
            specialties: t.1
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
