use std::io::Read;
use std::path::Path;
use std::{error::Error, fs::File};

use base64::Engine;
use base64::engine::general_purpose;
use chrono::NaiveDate;
use clap::Parser;
use reqwest::blocking::Client;
use serde::Serialize;

const API_URL: &str = "http://0.0.0.0:3000";

#[derive(Parser, Debug)]
struct Cli {
    image_path: String,
    date: String,
}

#[derive(Serialize)]
struct ImageRequest {
    date: String,
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

fn post_image(image_path: String, date: String) -> Result<(), Box<dyn Error>> {
    let mut image = File::open(&image_path)?;
    let mut buffer = Vec::new();
    image.read_to_end(&mut buffer)?;

    let base64_image = general_purpose::STANDARD.encode(&buffer);
    let image_request = ImageRequest {
        date: date.clone(),
        image: base64_image,
    };

    let client = Client::new();
    client.post(API_URL).json(&image_request).send()?;

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

    post_image(image_path, date)
}
