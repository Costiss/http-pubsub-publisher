use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod google_cloud;
mod utils;

async fn enqueue_pubsub(
    headers: HeaderMap,
    Path(topic): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tokio::spawn(async move {
        //Run the publisher on a background thread, reducing request lantency
        let parsed_headers = utils::axum_utils::headermap_to_hashmap(&headers);
        google_cloud::pubsub::enqueue(topic, payload, parsed_headers).await;
    });
    (StatusCode::ACCEPTED, Json(json!("enqueued")))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/:topic", post(enqueue_pubsub))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
