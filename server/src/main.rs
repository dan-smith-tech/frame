use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ImageResponse {
    message: String,
}

#[derive(Deserialize)]
struct ImageRequest {
    image: String,
}

async fn get_image() -> Json<ImageResponse> {
    let image = ImageResponse {
        message: "Hello, world!".to_string(),
    };
    Json(image)
}

async fn post_image(Json(payload): Json<ImageRequest>) -> Json<ImageResponse> {
    let response = ImageResponse {
        message: format!("Received image: {}", payload.image),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_image).post(post_image));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
