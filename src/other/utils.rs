use std::time::SystemTime;

use jsonwebtoken::{encode, Encoder, Algorithm};
use serde::Deserialize;
use sqlx_core::types::chrono::NaiveDate;

use crate::models::form_models::Claims;

pub fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

fn generate_jwt_token(claims: &Claims, secret_key: &str) -> String {
    let mut claims = claims.to_owned();
    let current_time = SystemTime::now();
    claims.expires_at = Some(current_time.unixt() + 60 * 60 * 1000); // Устанавливаем время истечения токена на 10 секунд
    let encoder = Encoder::new(Algorithm::HS256);
    let token = encode(&claims, &secret_key, &encoder).unwrap();
    token
}