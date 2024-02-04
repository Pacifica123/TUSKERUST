use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use sqlx_core::pool;

use crate::models;

pub async fn register(
    form: web::Json<models::RegistrationRequest::RegistrationRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder
// Result<HttpResponse, actix_web::Error> 
{
    // лог регистрационных данных в консоль
    println!("Registration request: {:?}", form);

    let result = sqlx::query!(
        r#"
        INSERT INTO simple_users (name, email, password) VALUES ($1, $2, $3)
        "#,
        form.username,
        form.email,
        form.password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => web::Json(models::RegistrationRequest::RegistrationResponse {
            success: true,
            message: "User registered successfully".to_string(),
        }),
        Err(_) => web::Json(models::RegistrationRequest::RegistrationResponse {
            success: false,
            message: "Failed to register user".to_string(),
        }),
    }
}


pub async fn login(
    form: web::Json<models::LoginRequest::LoginRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder
// Result<HttpResponse, actix_web::Error> 
{
    // лог регистрационных данных в консоль
    println!("Login request: {:?}", form);
    let result = sqlx::query!(
        r#"
        SELECT * FROM simple_users WHERE email = $1 AND password = $2
        "#,
        form.email,
        form.password
    )
    .fetch_one(pool.get_ref())
    .await;
    
    match result {
        Ok(_) => web::Json(models::LoginRequest::LoginResponse {
            success: true,
            message: "thsisistokenforemail_".to_string() + &form.email,
        }),
        Err(_) => web::Json(models::LoginRequest::LoginResponse {
            success: false,
            message: "Failed to login user".to_string(),
        }),
        
    }
}