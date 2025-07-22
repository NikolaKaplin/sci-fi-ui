use crate::{handlers::ai::urls::v1, utils::http_client::HTTP_CLIENT};
use reqwest::{self};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Image,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ai {
    pub name: String,
    pub description: String,
    pub provider: String,
    pub uncensored: bool,
    pub tier: String,
    pub community: bool,
    pub input_modalities: Vec<Modality>,
    pub output_modalities: Vec<Modality>,
    pub tools: bool,
    pub vision: bool,
    pub audio: bool,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Network request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API returned error: {0}")]
    ApiError(String),

    #[error("JSON parsing failed: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[tauri::command]
pub async fn list_models() -> Result<Vec<Ai>, ApiError> {
    let client = {
        let guard = HTTP_CLIENT
            .read()
            .map_err(|e| format!("Lock error: {}", e))
            .unwrap();
        guard.clone() // Client реализует Clone
    };

    let response = client.get(v1::LIST_MODELS).send().await?;

    if !response.status().is_success() {
        let error_msg = format!(
            "Status: {}, Body: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
        return Err(ApiError::ApiError(error_msg));
    }

    let models: Vec<Ai> = response.json().await?;

    Ok(models)
}

pub struct TextAi {
    pub prompt: String,
    pub model: String,
    pub seed: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub json: Option<bool>,
    pub system: Option<String>,
    pub stream: Option<bool>,
    pub private: Option<bool>,
    pub referrer: Option<String>,
}
