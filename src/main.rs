use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new().route("/vets", get(vets_get))
}

async fn vets_get() -> impl IntoResponse {
    let vets = vec![Vet {
        id: 1,
        first_name: String::from("James"),
        last_name: String::from("Carter"),
        specialties: vec![Specialty {
            id: 1,
            name: String::from("radiology"),
        }],
    }];

    Json(vets)
}

#[derive(Serialize)]
struct Vet {
    id: u64,
    first_name: String,
    last_name: String,
    specialties: Vec<Specialty>,
}

#[derive(Serialize)]
struct Specialty {
    id: u64,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::Value;
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn vets_get_returns() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/vets").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let first_vet = body.get(0);
        assert!(first_vet.is_some());
        assert!(first_vet.unwrap().get("id").is_some());
    }
}
