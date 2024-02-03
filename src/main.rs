mod routes;
mod db;
mod models;
pub mod handlers;
mod other;
use std::env;

use actix_web::{App, HttpServer, web::{self, Data}};

use sqlx::postgres::PgPoolOptions;
use log::{info, error};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // Создаем пул соединений с базой данных
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set").as_str())
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::all_routes::config_routes)
            // .wrap_fn(|req, srv| user_handler::auth_middleware(req, srv, Data::new(db_pool.clone())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}