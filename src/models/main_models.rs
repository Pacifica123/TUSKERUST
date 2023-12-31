use serde::{Deserialize, Serialize};
use sqlx_core::types::chrono::NaiveDate;

use crate::other::utils;

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    title: String,
    description: String,
}

#[derive(Debug, serde::Deserialize, sqlx::FromRow)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(deserialize_with = "utils::deserialize_naive_date")]
    pub creation_date: NaiveDate,
    pub is_email_verified: Option<bool>,
    pub is_banned: Option<bool>,
    pub is_deleted: Option<bool>,   
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Taskboard {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub owner_id: i32,
    pub create_time: String,
    pub access_link: String,
}

