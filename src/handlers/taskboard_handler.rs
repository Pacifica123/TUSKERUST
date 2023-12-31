use actix_web::{Responder, web, HttpResponse};
use log::info;
use sqlx::PgPool;

use crate::{models, db};
pub async fn create(
    form: web::Json<models::form_models::TaskboardForm>, 
    conn: web::Data<PgPool>
) -> impl Responder{
    info!("Создание задачи: {}", form.title);
    let title = &form.title;
    let description = &form.description;
    let user_id = &form.user_id;
    let taskboard_form = models::form_models::TaskboardForm {
        title: title.to_string(),
        description: description.to_string(),
        user_id: *user_id,
    };
       
    match db::taskboards::create_taskboard(conn, web::Json(taskboard_form)).await {
        Ok(_) => HttpResponse::Ok().content_type("text/html, charset=utf-8").body("Регистрация прошла успешно!"),
        Err(_) => HttpResponse::InternalServerError().content_type("text/html, charset=utf-8").body("Регистрация не удалась!"),
    }
}