use actix_web::{get, Responder};

#[get("/")]
pub async fn index_service() -> impl Responder {
    "Hello, World!"
}
