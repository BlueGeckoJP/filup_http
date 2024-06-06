use std::{fs, path::PathBuf};

use actix_web::{get, post, Error, HttpRequest, HttpResponse};
use tera::Context;

use crate::TEMPLATES;

#[get("/download")]
pub async fn download() -> Result<HttpResponse, Error> {
    let entries = fs::read_dir("./files").unwrap();
    let files_vec: Vec<String> = entries
        .into_iter()
        .map(|e| e.unwrap().path().to_string_lossy().to_string())
        .collect();
    let mut context = Context::new();
    context.insert("files", &files_vec);
    let view = TEMPLATES
        .render("download.html", &context)
        .expect("Failed to load TEMPLATES (templates/download.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/api/download")]
pub async fn api_download(req: HttpRequest, filename: String) -> HttpResponse {
    let file_path = PathBuf::from("./files").join(filename);

    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

    file.into_response(&req)
}
