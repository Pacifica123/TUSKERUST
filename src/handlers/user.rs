use actix_web::{web, Responder};
use sqlx::PgPool;

use crate::models;

pub async fn user_info(
    pool: web::Data<PgPool>,
    token: String
) -> impl Responder{
    let email = token.replace("thsisistokenforemail_", "");
    let result = sqlx::query!(
        r#"
        SELECT * FROM simple_users WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await;
    
    // debug: 
    println!("{:?}", result);

    // Если все без ошибок то возвращаем информацию, если ошибка то возвращаем ошибку
    match result {
        Ok(user) => web::Json(models::User::UserInfo {
            success: true,
            name: user.name,
            email:user.email,
        })
        ,
        Err(_) => web::Json(models::User::UserInfo {
            success: false,
            name: None,
            email: None,
        })
    }
}