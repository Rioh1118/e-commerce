use actix_web::{App, HttpServer};

mod app;
mod controllers;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(app::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
