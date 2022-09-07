mod config;
mod game;
mod routes;

use std::io;

use actix::Actor;
use actix_files::Files;
use actix_session::storage::{CookieSessionStore, SessionStore};
use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use clap::Parser;
use log::LevelFilter;

use crate::config::Config;
use crate::game::server::WsServer;

/// Command-line arguments.
#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Args {
    /// The port to which the web server is bound.
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
    /// The verbosity level of the log.
    #[clap(short, long, default_value_t = LevelFilter::Info)]
    verbosity: LevelFilter,
}

/// Builds a middleware that manages sessions.
fn build_session_middleware(config: &Config) -> SessionMiddleware<impl SessionStore> {
    let session_secret =
        base64::decode(&config.session_secret).expect("cannot decode session secret from Base64");

    SessionMiddleware::builder(
        CookieSessionStore::default(),
        Key::derive_from(&session_secret),
    )
    .cookie_name("session".to_string())
    .build()
}

/// Launches the web server.
#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    env_logger::builder().filter_level(args.verbosity).init();

    let config = Config::new().expect("cannot read configuration");

    let game_server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_server.clone()))
            .wrap(Logger::default())
            .wrap(build_session_middleware(&config))
            .service(
                web::scope("/api")
                    .service(routes::login::post)
                    .service(routes::logout::post),
            )
            .service(Files::new("/", "../build").index_file("index.html"))
    })
    .bind(("127.0.0.1", args.port))?
    .run()
    .await
}
