#[derive(Debug, serde::Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}