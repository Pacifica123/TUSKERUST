use actix_web::web;
use  crate::handlers;



pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
                    .route("/registration", web::post().to(|form, conn| handlers::user_handler::registration(form, conn)))
                    
    )
    .service(
        web::scope("/taskboards")
                    .route("/create", web::post().to(|form, conn| handlers::taskboard_handler::create(form, conn)))
                    // .route("/{taskboard_id}", web::get().to(|id, conn| handlers::taskboard_handler::read(id, conn)))
    );
}