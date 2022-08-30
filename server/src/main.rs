mod game;
mod routes;

use std::io;

use actix::Actor;
use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};

use crate::game::server::WsServer;

/// Returns a secret key that is used to encrypt the session cookie stored by the client.
///
/// # Panics
///
/// Panics if the environment variable `SESSION_SECRET` is not a base64-encoded string of at least
/// 32 randomly-generated bytes.
fn get_session_secret() -> Key {
    let session_secret = dotenv::var("SESSION_SECRET")
        .expect("environment variable `SESSION_SECRET` must be defined");
    let session_secret = base64::decode(session_secret)
        .expect("cannot decode environment variable `SESSION_SECRET` from base64");
    Key::derive_from(&session_secret)
}

/// Launches the web server.
#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let game_server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_server.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), get_session_secret())
                    .cookie_name("session".to_string())
                    .build(),
            )
            .service(
                web::scope("/api")
                    .service(routes::login::post)
                    .service(routes::logout::post),
            )
            .service(Files::new("/", "../build").index_file("index.html"))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
