use actix_web::web;
use  crate::handlers;



pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/rwtm/api/v0")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("userinfo", web::get().to(handlers::user::user_info))
    );
}