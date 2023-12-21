use serde::Deserialize;

#[derive(Debug, serde::Deserialize)]
pub struct Task {
    title: String,
    description: String,
}
