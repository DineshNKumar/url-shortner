use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct GenerateRequestBody {
    pub original_url: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponseBody {
    pub base_url: String,
}