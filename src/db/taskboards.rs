use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use sqlx_core::{Error, types::{chrono::Utc}};
use uuid::Uuid;
use crate::models;

pub async fn create_taskboard(
    conn: web::Data<PgPool>, 
    taskboard_form: web::Json<models::form_models::TaskboardForm>
) -> Result<HttpResponse, Error>
{
    let title = &taskboard_form.title;
    let description = &taskboard_form.description;
    let create_time = Utc::now().naive_utc().date().to_string();
    let access_link  = "https://localhost:8080/taskboards/".to_string() + &Uuid::new_v4().to_string();
    let user_id = &taskboard_form.user_id;
    let id: i32 = sqlx::query!(
        r#"INSERT INTO taskboards (title, description, owner_id) VALUES ($1, $2, $3) RETURNING id"#,
        title,
        description,
        user_id
    ).fetch_one(conn.get_ref())
    .await?.id;
    Ok(HttpResponse::Ok().json(id))
}

pub async fn read_taskboards(conn: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let taskboards = sqlx::query!(
        r#"SELECT * FROM taskboards"#,
    )
    .fetch_all(conn.get_ref())
    .await?
    .into_iter()
    .map(|row| models::main_models::Taskboard {
        id: row.id,
        title: row.title.unwrap_or_default(),
        description: row.description.unwrap_or_default(),
        owner_id: row.owner_id.unwrap_or_default(),
        create_time: row.create_time.unwrap_or_default().to_string(),
        access_link: row.access_link.unwrap_or_default(),
    })
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(taskboards))
}

pub async fn delete_taskboard(
    conn: web::Data<PgPool>, 
    id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    sqlx::query!(
        r#"DELETE FROM taskboards WHERE id = $1"#,
        *id
    )
    .execute(conn.get_ref())
    .await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn update_taskboard(
    conn: web::Data<PgPool>, 
    id: web::Path<i32>,
    taskboard_form: web::Json<models::form_models::TaskboardForm>
) -> Result<HttpResponse, Error> {
    sqlx::query!(
        r#"UPDATE taskboards SET title = $1, description = $2 WHERE id = $3"#,
        taskboard_form.title,
        taskboard_form.description,
        *id
    )
    .execute(conn.get_ref())
    .await?;
    Ok(HttpResponse::Ok().finish())
}

