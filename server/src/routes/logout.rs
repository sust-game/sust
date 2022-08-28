use actix_web::{get, HttpResponse, Responder};

#[get("/logout")]
pub async fn handler() -> impl Responder {
    HttpResponse::Ok()
}
