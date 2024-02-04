#[derive(Debug, serde::Serialize)]
pub struct GeneralError {
    pub success: bool,
    pub message: String,
}