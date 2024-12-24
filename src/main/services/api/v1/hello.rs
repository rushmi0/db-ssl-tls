use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{get, web, HttpResponse, Responder};
use miniserde::{json, Serialize};
use crate::module::greet;

#[derive(Serialize)]
struct Greeting {
    timestamp: u32,
    message: String,
}

#[get("/{name}")]
pub async fn hello_service(name: web::Path<String>) -> impl Responder {
    let greeting_message: String = greet(&name).await;
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    let response = Greeting {
        timestamp: created_at,
        message: greeting_message,
    };

    let serialized_response = json::to_string(&response);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized_response)
}
