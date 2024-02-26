use actix_web::web;
use  crate::handlers;



pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/rwtm/api/v0")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("/userinfo", web::get().to(handlers::user::user_info))
                            .route("/taskboards", web::get().to(handlers::taskboard::get_taskboards))
                            .route("/taskboards", web::post().to(handlers::taskboard::add_taskboard))
                            .route("/taskboards/{taskboard_id}", web::delete().to(handlers::taskboard::delete_taskboard))
    );
}