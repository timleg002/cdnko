use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String
}

#[derive(Deserialize, Serialize)]
pub struct FileResponse {
    pub file_names: Vec<String>
}