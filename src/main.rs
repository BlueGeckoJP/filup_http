mod apis;
mod services;
mod debug_hotreload;

#[macro_use]
extern crate log;

use std::{error::Error, fs, io::{self}, sync::Mutex};

use actix_multipart::form::{tempfile::TempFileConfig, MultipartFormConfig};
use actix_web::{middleware::Logger, App, HttpServer};
use clap::Parser;
use tera::Tera;
use once_cell::sync::{Lazy, OnceCell};

use crate::{apis::*, services::*, debug_hotreload::debug_hotreload};

pub static TEMPLATES: Templates = Templates { t: OnceCell::new() };
pub static SAVE_DIRECTORY: Lazy<String> = Lazy::new(|| String::from("./files"));

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server port
    #[arg(short, long, default_value_t = 8080)]
    port: u16
}

pub struct Templates {
    t: OnceCell<Mutex<Tera>>
}

impl Templates {
    fn update(&self) -> Result<(), Box<dyn Error>> {
        let tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
        };
        {
            if self.t.get().is_none() {
                self.t.set(Mutex::new(tera)).unwrap();
            } else {
                let mut t = self.t.get().unwrap().lock().unwrap();
                *t = tera;
            }
        }
        Ok(())
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    TEMPLATES.update().unwrap();

    let args = Args::parse();
    info!("Arguments: {:?}", args);

    info!("Creating SAVE_DIRECTORY in progress");
    fs::create_dir_all(SAVE_DIRECTORY.clone())?;

    if cfg!(debug_assertions) {
        info!("Debugging is enabled");
        debug_hotreload();
    }

    info!("Starting HTTP server at '0.0.0.0:{}'", args.port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(MultipartFormConfig::default().total_limit(10 * 1024 * 1024 * 1024)) // 10GB
            .app_data(TempFileConfig::default().directory(SAVE_DIRECTORY.clone()))
            .service(actix_files::Files::new("/files/raw", "./files").show_files_listing())
            .service(actix_files::Files::new("/assets", "./assets").show_files_listing())
            .service(service_root::root)
            .service(service_upload::upload)
            .service(service_files::files)
            .service(api_upload::upload)
            .service(api_remove::remove)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
