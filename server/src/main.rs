use axum::{Json, Router, routing::get};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};

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

async fn get_image() -> Json<ImageResponse> {
    let db = sled::open(DB_PATH);
    match db {
        Ok(db) => {
            let mut keys: Vec<String> = db
                .iter()
                .keys()
                .filter_map(|item| match item {
                    Ok(itm) => String::from_utf8(itm.to_vec()).ok(),
                    Err(_) => None,
                })
                .collect();
            keys.sort();

            let chosen_key: Option<&String> = keys.iter().reduce(|acc, key| {
                // if key (which is a date) is not in the future
                if let Ok(date) = chrono::NaiveDate::parse_from_str(key, "%Y-%m-%d") {
                    if date > chrono::Local::now().naive_local().date() {
                        return acc;
                    }
                }
                key
            });

            if let Some(chosen_key) = chosen_key {
                let image_data = db.get(chosen_key.as_bytes());
                let base64_image = match image_data {
                    Ok(Some(data)) => general_purpose::STANDARD.encode(data),
                    Ok(None) => {
                        eprintln!("No image found for date: {}", chosen_key);
                        return Json(ImageResponse {
                            image: String::new(),
                        });
                    }
                    Err(e) => {
                        eprintln!("Error retrieving image: {}", e);
                        return Json(ImageResponse {
                            image: String::new(),
                        });
                    }
                };

                Json(ImageResponse {
                    image: base64_image,
                })
            } else {
                println!("No valid keys found.");
                return Json(ImageResponse {
                    image: String::new(),
                });
            }
        }
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            Json(ImageResponse {
                image: String::new(),
            })
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
