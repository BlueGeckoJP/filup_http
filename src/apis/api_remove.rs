use std::{fs, path::Path};

use actix_web::{post, Error, HttpResponse};

#[post("/api/remove")]
pub async fn remove(filename: String) -> Result<HttpResponse, Error> {
    let path = Path::new("./files/").join(&filename);
    if !path.exists() {
        return Ok(HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body("File did not exist")
            .into());
    }
    fs::remove_file(&path).unwrap();
    info!("File removed by API: {}", path.display().to_string());

    Ok(HttpResponse::Ok().into())
}
