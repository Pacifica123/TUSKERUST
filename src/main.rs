mod routes;
mod db;
mod models;
pub mod handlers;
mod other;
use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web::{self, Data}};

use sqlx::postgres::PgPoolOptions;
use log::{info, error};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // Создаем пул соединений с базой данных
    // let db_pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set").as_str())
    //     .await
    //     .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:4200") // Разрешенный origin
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION, 
                    actix_web::http::header::ACCEPT, 
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::ORIGIN,
                    actix_web::http::header::X_CONTENT_TYPE_OPTIONS,
                    actix_web::http::header::X_FRAME_OPTIONS,
                    actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN
                    ])
                .max_age(3600),

            )
            // .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::all_routes::config_routes)
            // .wrap_fn(|req, srv| user_handler::auth_middleware(req, srv, Data::new(db_pool.clone())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}