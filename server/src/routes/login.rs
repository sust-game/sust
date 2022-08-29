use actix_session::Session;
use actix_web::{post, web::Json, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostBody {
    username: String,
    // TODO: Validate credentials using the password
    #[allow(dead_code)]
    password: String,
}

#[post("/login")]
pub async fn post(session: Session, body: Json<PostBody>) -> impl Responder {
    session.insert("username", body.username.clone()).ok();
    HttpResponse::Ok().json(&*session.entries())
}
