use std::time::{SystemTime, UNIX_EPOCH};

use async_recursion::async_recursion;
use axum::{Router, routing::get, response::{Html, IntoResponse}, body::StreamBody};
use rand::{thread_rng, seq::SliceRandom};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use walkdir::{WalkDir, DirEntry};

pub mod web;
use web::web_page::get_index;

const FILE_TYPES: &[&str] = &["avif", "heic", "jpeg", "jpg", "png", "webpg"];

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let mut photos = get_all_photos();
    let mut rng = thread_rng();
    photos.shuffle(&mut rng);

    let landing_page = get_index();
    let app = Router::new()
        .route("/", get(|| async { Html(landing_page) }))
        .route("/next", get(|| async { stream_next_photo(photos).await }));

    println!("Visit http://localhost:4015 in your browser.");
    axum::Server::bind(&"0.0.0.0:4015".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn get_all_photos() -> Vec<DirEntry> {
    let mut photos: Vec<DirEntry> = vec![];

    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();

        let extension_opt = f_name.split('.').last();
        if extension_opt.is_none() {
            continue;
        }

        let extension = extension_opt.unwrap();

        if !FILE_TYPES.iter().any(|&ext| ext == extension) {
            continue;
        }

        photos.push(entry);
    }

    println!("Found {} photos.", photos.len());
    photos
}

async fn stream_next_photo(photos: Vec<DirEntry>) -> impl IntoResponse {
    let photo_file = get_next_photo_data(photos).await;
    let stream = ReaderStream::new(photo_file);
    StreamBody::new(stream)
}

#[async_recursion]
async fn get_next_photo_data(mut photos: Vec<DirEntry>) -> File {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("System time is before 1970");
    let three_second_periods = (since_the_epoch.as_secs() as usize) / 3;
    let corresponding_ix = three_second_periods % photos.len();
    let corresponding_photo = 
        &photos[corresponding_ix];

    match File::open(corresponding_photo.path()).await {
        Ok(file) => file,
        Err(_) => {
            photos.remove(corresponding_ix);
            get_next_photo_data(photos).await
        }
    }
}