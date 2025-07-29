use axum::{Json, Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use serde_json::json;

const DB_PATH: &str = ".images";

#[derive(Deserialize)]
struct ImageRequest {
    image: String,
    date: String,
}

#[derive(Serialize)]
struct ImageResponse {
    image: String,
}

fn get_image_inner() -> Result<ImageResponse, String> {
    let db = sled::open(DB_PATH).map_err(|e| e.to_string())?;

    let mut keys: Vec<String> = db
        .iter()
        .keys()
        .filter_map(|item| match item {
            Ok(itm) => String::from_utf8(itm.to_vec()).ok(),
            Err(_) => None,
        })
        .collect();
    keys.sort();

    let chosen_key = keys
        .iter()
        .reduce(|previous_date, key| {
            // return previous data if the new date is NOT in the future
            if let Ok(date) = chrono::NaiveDate::parse_from_str(key, "%Y-%m-%d") {
                if date > chrono::Local::now().naive_local().date() {
                    return previous_date;
                }
            }
            key
        })
        .ok_or("No valid keys found".to_string())?;

    let image_data = db
        .get(chosen_key.as_bytes())
        .map_err(|e| e.to_string())?
        .ok_or("No image found for the chosen date".to_string())?;
    let base64_image = String::from_utf8(image_data.to_vec()).map_err(|e| e.to_string())?;

    Ok(ImageResponse {
        image: base64_image,
    })
}

async fn get_image() -> Result<Json<ImageResponse>, (StatusCode, Json<serde_json::Value>)> {
    match get_image_inner() {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_response = json!({ "error": e });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

async fn post_image(Json(payload): Json<ImageRequest>) {
    let image = payload.image.clone();
    let date = payload.date.clone();

    let db = sled::open(DB_PATH);
    match db {
        Ok(db) => {
            if let Err(e) = db.insert(date.clone().into_bytes(), image.clone().into_bytes()) {
                eprintln!("Error inserting data: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error opening database: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_image).post(post_image));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
