use actix_web::{get, Error, HttpResponse};
use tera::Context;

use crate::TEMPLATES;

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error> {
    let view = TEMPLATES
        .render("root.html", &Context::new())
        .expect("Failed to load TEMPLATES (templates/root.html)");
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}
