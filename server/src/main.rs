mod routes;

use std::io;

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::login::handler)
            .service(routes::logout::handler)
            .service(Files::new("/", "../build").index_file("index.html"))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
