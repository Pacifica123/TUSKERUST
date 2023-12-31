
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sqlx_core::types::chrono;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RegForm{
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TaskboardForm{
    pub title: String,
    pub description: String,//имеется ввиду краткое описание в миниатюре
    pub user_id: i32, //TODO подумать над тем как в запросе будет передаваться user_id
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}
impl Claims {
    pub fn new(email: String) -> Self {
        Claims {
            email,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginFormData {
    pub username: String,
    pub password: String,
}