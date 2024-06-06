use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, Error, HttpResponse, Responder};
use log::info;
use tera::Context;

use crate::{SAVE_DIRECTORY, TEMPLATES};

#[derive(Debug, MultipartForm)]
struct UploadApiForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[get("/upload")]
pub async fn upload() -> Result<HttpResponse, Error> {
    let view = TEMPLATES
        .render("upload.html", &Context::new())
        .expect("Failed to load TEMPLATES (templates/upload.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/api/upload")]
pub async fn api_upload(
    MultipartForm(form): MultipartForm<UploadApiForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("{}/{}", SAVE_DIRECTORY.clone(), f.file_name.unwrap());
        info!("Uploaded file to: {}", path);
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}
