use actix_web::{get, HttpResponse, Responder};

#[get("/login")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok()
}
