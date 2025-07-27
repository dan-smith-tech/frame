use std::error::Error;
use std::path::Path;

use chrono::NaiveDate;
use clap::Parser;
use image::ImageReader;
use reqwest::blocking::Client;
use serde::Serialize;

const IMAGE_DIR: &str = ".images";
const API_URL: &str = "http://0.0.0.0:3000";

#[derive(Parser, Debug)]
struct Cli {
    image_path: String,
    date: String,
}

#[derive(Serialize)]
struct ImageRequest {
    image: String,
}

fn is_valid_image(image_path: &str) -> bool {
    let path = Path::new(image_path);

    if !path.exists() {
        return false;
    }

    if let Some(ext) = path.extension() {
        if ext != "jpg" && ext != "png" && ext != "jpeg" {
            return false;
        }
    }

    true
}

fn is_valid_date(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

fn post_image() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let payload = ImageRequest {
        image: "this is an image".to_string(),
    };

    let response = client.post(API_URL).json(&payload).send()?;

    println!("Response: {:?}", response.status());
    println!("Response body: {:?}", response.text()?);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let image_path = args.image_path;
    let date = args.date;

    if !is_valid_image(&image_path) {
        eprintln!("Invalid image file. Supported formats are jpg, jpeg, and png.");
        std::process::exit(1);
    }

    if !is_valid_date(&date) {
        eprintln!("Invalid date format. Expected YYYY-MM-DD.");
        std::process::exit(1);
    }

    post_image()
}
