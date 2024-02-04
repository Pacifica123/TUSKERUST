#[derive(Debug, serde::Serialize)]
pub struct UserInfo {
    pub success: bool,
    pub name: Option<String>,
    pub email: Option<String>,
}