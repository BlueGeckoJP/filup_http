use std::fs;

use actix_web::{get, Error, HttpResponse};
use tera::Context;

use crate::TEMPLATES;

#[get("/files")]
pub async fn files() -> Result<HttpResponse, Error> {
    let entries = fs::read_dir("./files").unwrap();
    let files_vec: Vec<String> = entries
        .into_iter()
        .map(|e| e.unwrap().path().to_string_lossy().to_string())
        .collect();
    let mut context = Context::new();
    context.insert("files", &files_vec);
    let view = TEMPLATES
        .render("files.html", &context)
        .expect("Failed to load TEMPLATES (templates/files.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
