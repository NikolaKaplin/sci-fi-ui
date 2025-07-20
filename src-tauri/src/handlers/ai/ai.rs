use crate::handlers::ai::urls::v1;
use reqwest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Image,
    Audio, // Другие модальности можно добавить по необходимости
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
    let response = reqwest::get(v1::LIST_MODELS).await?;

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
    pub seed: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub presence_penalty: f32,
    pub json: bool,
    pub system: String,
    pub stream: bool,
    pub private: bool,
    pub referrer: String,
}

// pub async fn generate_text()
