use actix_web::{web, Responder};
use sqlx::PgPool;

use crate::models;

pub async fn get_taskboards(
    pool: web::Data<PgPool>,
    token: String
) -> impl Responder{
    let email = token.replace("thsisistokenforemail_", "");
    // Получаем пользователя чтобы потом достать из него список досок
    let result = sqlx::query!(
        r#"
        SELECT * FROM simple_users WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await;
    
    let owner_id = result.unwrap().id;

    let result_taskboards = sqlx::query!(
        r#"
        SELECT * FROM taskboards_test WHERE owner_id = $1
        "#,
        owner_id
    )
    .fetch_all(pool.get_ref())
    .await;

    web::Json(models::Taskboard::ListTaskboards {
        success: result_taskboards.is_ok(),
        taskboards: result_taskboards.as_ref().map_or_else(|_| vec![], |taskboards| taskboards.iter().map(|taskboard| models::Taskboard::Taskboard::new(taskboard.id, taskboard.name.clone().unwrap_or_default(), taskboard.owner_id.unwrap_or_default())).collect())
    })

}