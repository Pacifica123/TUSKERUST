use log::info;
use sqlx::PgPool;
use sqlx_core::{Error, types::chrono::{Utc, NaiveDate}};

use crate::models;


pub async fn create_user(conn: PgPool, name: &str, email: &str, pass: &str) -> Result<i32, Error>{
    info!("Создание пользователя: {}", name);
    let current_date: NaiveDate = Utc::now().naive_utc().date();
    let id: i32 = sqlx::query!(
        r#"INSERT INTO users (name, email, password, creation_date) VALUES ($1, $2, $3, $4) RETURNING id"#,
        name,
        email,
        pass,
        current_date
    ).fetch_one(&conn)
    .await?.id;
    sqlx::query!(
        r#"INSERT INTO profiles (user_id) VALUES ($1)"#,
        id
    ).execute(&conn)
    .await?;
    Ok(id)
}

pub async fn read_user_by_email(
    conn: PgPool, 
    email: &str,
    pass: &str
) -> Result<models::main_models::User, Error> {
    let user = sqlx::query_as!(
        models::main_models::User,
        r#"SELECT * FROM users WHERE email = $1 AND password = $2"#,
        email,
        pass
    ).fetch_one(&conn)
    .await?;
    Ok(user)
}
