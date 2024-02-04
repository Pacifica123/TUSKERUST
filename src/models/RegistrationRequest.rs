#[derive(Debug, serde::Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct RegistrationResponse {
    pub success: bool,
    pub message: String,
}