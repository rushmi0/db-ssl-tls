use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use dotenvy::dotenv;
use std::env;

use crate::services::api::v1;
use crate::storage;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    storage::init_db();
    env_logger::init();

    // ดึงค่า IP และ PORT จาก .env หรือใช้ค่าดีฟอลต์
    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid u16");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors_config())
            .configure(v1::service_hub)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3700)
        .send_wildcard()
        .allow_any_origin()
}