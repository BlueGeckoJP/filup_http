use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, Error, HttpResponse, Responder};

use crate::SAVE_DIRECTORY;

#[derive(Debug, MultipartForm)]
struct UploadApiForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/api/upload")]
pub async fn upload(
    MultipartForm(form): MultipartForm<UploadApiForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("{}/{}", SAVE_DIRECTORY.clone(), f.file_name.unwrap());
        info!("Uploaded file to: {}", path);
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}
