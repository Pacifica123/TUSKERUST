use actix_web::{web, HttpResponse, Responder};
use crate::models;

pub async fn index() -> impl Responder{
    HttpResponse::Ok().body("Добро пожаловать в ТАСКЕР!")
}


pub async fn add_task(params: web::Either<web::Form<models::Task>, web::Query<models::Task>> ) -> impl Responder{
    match params {
        web::Either::Left(task) => {
            println!("Задача: {:?} успешно создана!", task);
        }
        web::Either::Right(query) => {
            println!("Задача: {:?} успешно создана!", query);
        }
    }
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Задача добавлена!")
}
