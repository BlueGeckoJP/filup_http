use actix_web::{get, Error, HttpResponse};
use tera::Context;

use crate::TEMPLATES;

#[get("/upload")]
pub async fn upload() -> Result<HttpResponse, Error> {
    let view = TEMPLATES
        .render("upload.html", &Context::new())
        .expect("Failed to load TEMPLATES (templates/upload.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
