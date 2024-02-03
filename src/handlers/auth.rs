use actix_web::{web, HttpResponse};

use crate::models;

pub async fn register(req: web::Json<models::RegistrationRequest::RegistrationRequest>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json("Register"))
}