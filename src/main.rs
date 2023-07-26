use std::{
    path::PathBuf,
    time::{Duration, SystemTime},
};

use actix_web::{
    get,
    http::{
        header::{
            ContentDisposition, DispositionParam, DispositionType, HttpDate, TryIntoHeaderValue,
            EXPIRES,
        },
        StatusCode,
    },
    web, App, HttpRequest, HttpResponse, HttpServer, Result, middleware,
};
use rand::{seq::SliceRandom, thread_rng};
use walkdir::{DirEntry, WalkDir};

pub mod webpage;
use webpage::webpage::get_index;

const FILE_TYPES: &[&str] = &["avif", "heic", "jpeg", "jpg", "png", "webp"];

struct AppState {
    photos: Vec<DirEntry>,
    server_start: SystemTime,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut photos = get_all_photos();
    let mut rng = thread_rng();
    photos.shuffle(&mut rng);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::Data::new(AppState {
                photos: photos.to_owned(),
                server_start: SystemTime::now(),
            }))
            .service(index)
            .service(next)
    })
    .bind(("0.0.0.0", 4015))?
    .run();

    println!("Visit http://localhost:4015 in your browser.");
    server.await
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let landing_page = get_index();
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(landing_page))
}

#[get("/next")]
async fn next(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let photo_path = &get_next_photo_data(&data.photos, data.server_start);
    let extension = photo_path
        .extension()
        .map(|ext| ext.to_string_lossy().to_ascii_lowercase());
    let mut file = actix_files::NamedFile::open_async(photo_path)
        .await
        .unwrap();

    if extension.unwrap_or_default() == "heic" {
        file = file.set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![DispositionParam::Filename(
                photo_path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            )],
        });
        file = file.set_content_type("image/heic".parse().unwrap());
    }

    // file.set_content_disposition(ContentDisposition::)

    let mut response = file.into_response(&req);
    let headers = response.head_mut();
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(4))
        .unwrap();
    let http_date = HttpDate::from(expiration);
    headers
        .headers
        .append(EXPIRES, http_date.try_into_value().unwrap());

    response
}

fn get_all_photos() -> Vec<DirEntry> {
    let mut photos: Vec<DirEntry> = vec![];

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
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

    if photos.is_empty() {
        panic!("No photos found. Exiting.");
    }

    photos
}

fn get_next_photo_data(photos: &Vec<DirEntry>, base_time: SystemTime) -> PathBuf {
    let now = SystemTime::now();
    let since_server_start = now
        .duration_since(base_time)
        .expect("System time is before server start time");
    let three_second_periods = (since_server_start.as_secs() as usize) / 4;
    let corresponding_ix = three_second_periods % photos.len();
    let corresponding_photo = &photos[corresponding_ix];
    let path = corresponding_photo.path();

    path.to_owned()
}
