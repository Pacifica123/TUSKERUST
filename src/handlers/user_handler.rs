use actix_web::HttpRequest;
use actix_web::dev::{ServiceResponse, ServiceRequest, AppConfig, Service};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use log::error;
use sqlx::PgPool;
use crate::db::users;
use crate::models;
use crate::models::form_models::Claims;
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use actix_web::error::Error;
use std::env;

pub async fn registration(
    form: web::Form<models::form_models::RegForm>, 
    conn: web::Data<PgPool>
) -> impl Responder{
    info!("Регистрация пользователя: {}", form.name);
    let name = &form.name;
    let email = &form.email;
    let password = &form.password;
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    match users::create_user(conn.get_ref().clone(), name, email, &password).await {
        Ok(_) => HttpResponse::Ok().content_type("text/html, charset=utf-8").body("Регистрация прошла успешно!"),
        Err(e) => {
            error!("Registration failed: {}", e);
            HttpResponse::InternalServerError().content_type("text/html, charset=utf-8").body("Регистрация не удалась!")
        },
    }
}

pub async fn auth_middleware(
    req: ServiceRequest,
    srv: actix_web::dev::Service,
    db_pool: web::Data<PgPool>,
) -> Result<ServiceResponse, Error> {
    let authorization_header = req.headers().get("Authorization");
    let secret_value = env::var("JWT_SECRET_KEY")
        .expect("JWT_SECRET_KEY not found in environment variables");
    let secret = secret_value.as_bytes();

    if let Some(authorization) = authorization_header {
        if let Ok(token) = authorization.to_str() {
            let secret = secret; 
            let validation = Validation::new(Algorithm::HS256);

            match decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation) {
                Ok(_) => Ok(ServiceResponse::new(req, HttpResponse::build(StatusCode::OK).finish())),
                Err(_) => Ok(ServiceResponse::new(req, HttpResponse::build(StatusCode::UNAUTHORIZED).finish())),
            }
        } else {
            Ok(ServiceResponse::new(req, HttpResponse::build(StatusCode::UNAUTHORIZED).finish()))
        }
    } else {
        Ok(ServiceResponse::new(req, HttpResponse::build(StatusCode::UNAUTHORIZED).finish()))
    }
}
