use actix_web::{web, FromRequest, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models;

pub async fn get_owner_id_from_email(pool: web::Data<PgPool>, email: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id FROM simple_users WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(result.id)
}


pub async fn get_taskboards(
    pool: web::Data<PgPool>,
    token: String
) -> impl Responder{
    let email = token.replace("thsisistokenforemail_", "");
    
    let owner_id = get_owner_id_from_email(pool.clone(), email.as_str()).await;

    let result_taskboards = sqlx::query!(
        r#"
        SELECT * FROM taskboards_test WHERE owner_id = $1
        "#,
        owner_id.unwrap_or_default()
    )
    .fetch_all(pool.get_ref())
    .await;

    web::Json(models::Taskboard::ListTaskboards {
        success: result_taskboards.is_ok(),
        taskboards: result_taskboards.as_ref().map_or_else(|_| vec![], |taskboards| taskboards.iter().map(|taskboard| models::Taskboard::Taskboard::new(taskboard.id, taskboard.name.clone().unwrap_or_default(), taskboard.owner_id.unwrap_or_default())).collect())
    })

}

pub async fn add_taskboard(
    pool: web::Data<PgPool>,
    token: String,
    name: String
) -> impl Responder{
    let email = token.replace("thsisistokenforemail_", "");

    let owner_id = get_owner_id_from_email(pool.clone(), email.as_str()).await;
    let result = sqlx::query!(
        r#"
        INSERT INTO taskboards_test (name, owner_id) VALUES ($1, $2)
        "#,
        name,
        owner_id.unwrap_or_default()
    )
    .execute(pool.get_ref())
    .await;

    web::Json(models::Taskboard::ListTaskboards {
        success: result.is_ok(),
        taskboards: vec![]
    })
}

pub async fn delete_taskboard(
    pool: web::Data<PgPool>,
    token: String,
    path: web::Path<(i32,)>,
) -> impl Responder{
    let email = token.replace("thsisistokenforemail_", "");
    let owner_id = get_owner_id_from_email(pool.clone(), email.as_str()).await;
    let taskboard_id = path.0;
    let result = sqlx::query!(
        r#"
        DELETE FROM taskboards_test WHERE id = $1 AND owner_id = $2
        "#,
        taskboard_id,
        owner_id.unwrap_or_default()
    )
    .execute(pool.get_ref())
    .await;

    // Проверяем, сколько строк было удалено
    match result {
        Ok(ref r) if r.rows_affected() == 1 => {
            // Вернуть успешный ответ JSON
            web::Json(models::Taskboard::ListTaskboards {
                success: true,
                taskboards: vec![]
            })
        },
        _ => {
            // Если ни одна строка не была удалена или возникла ошибка,
            // вернуть ошибку сервера
            web::Json(models::Taskboard::ListTaskboards {
                success: false,
                taskboards: vec![]
            })
        }
    }
}