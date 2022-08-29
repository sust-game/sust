use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};

#[post("/logout")]
pub async fn post(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok()
}
