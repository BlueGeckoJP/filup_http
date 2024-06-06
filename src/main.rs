mod apis;
mod services;

use std::{fs, io};

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{middleware::Logger, App, HttpServer};
use lazy_static::lazy_static;
use log::info;
use tera::Tera;

use crate::services::{download_service, upload_service};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                panic!("Parsin error(s): {}", e);
            }
        };
        tera
    };
    pub static ref SAVE_DIRECTORY: String = String::from("./files");
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Creating SAVE_DIRECTORY in progress");
    fs::create_dir_all(SAVE_DIRECTORY.clone())?;

    info!("Starting HTTP server");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(TempFileConfig::default().directory(SAVE_DIRECTORY.clone()))
            .service(actix_files::Files::new("/files", "./files").show_files_listing())
            .service(upload_service::upload)
            .service(upload_service::api_upload)
            .service(download_service::download)
            .service(download_service::api_download)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
