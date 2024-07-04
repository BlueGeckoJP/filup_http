use std::fs;

use actix_web::{get, Error, HttpResponse};
use serde::Serialize;
use tera::Context;

use crate::TEMPLATES;

#[derive(Debug, Serialize)]
struct File {
    filename: String,
    path: String,
}

#[get("/files")]
pub async fn files() -> Result<HttpResponse, Error> {
    let entries = fs::read_dir("./files").unwrap();
    let files_vec: Vec<File> = entries
        .into_iter()
        .map(|e| {
            let filename = e.unwrap().file_name().to_string_lossy().to_string();
            let path = format!("/files/raw/{}", filename);
            File {
                filename: filename,
                path: path,
            }
        })
        .collect();
    let mut context = Context::new();
    context.insert("files", &files_vec);
    let view = TEMPLATES.t.get().unwrap().lock().unwrap()
        .render("files.html", &context)
        .expect("Failed to load TEMPLATES (templates/files.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
