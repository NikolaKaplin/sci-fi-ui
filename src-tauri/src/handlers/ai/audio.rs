use dotenv::dotenv;
use reqwest;
use std::env;
use urlencoding::encode;

use crate::utils::http_client::HTTP_CLIENT;

struct ApiConfig {
    token: String,
}

impl ApiConfig {
    fn new() -> Self {
        dotenv().ok();
        let token =
            env::var("TTS_API_TOKEN").expect("TTS_API_TOKEN must be set in environment variables");

        ApiConfig { token }
    }
}

#[tauri::command]
pub async fn generate_audio(
    text: String,
    voice: String,
    token: Option<String>,
) -> Result<Vec<u8>, String> {
    let api_config = ApiConfig::new();
    let api_token = token.unwrap_or(api_config.token);
    let encoded_text = encode(&text);
    let url = format!("https://text.pollinations.ai/{}", encoded_text);

    let client = {
        let guard = HTTP_CLIENT
            .read()
            .map_err(|e| format!("Lock error: {}", e))?;
        guard.clone() // Client реализует Clone
    };

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .query(&[("model", "openai-audio"), ("voice", &voice)])
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }

    if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
        if content_type != "audio/mpeg" {
            return Err(format!("Unexpected content type: {:?}", content_type));
        }
    }

    let audio_data = response.bytes().await.map_err(|e| e.to_string())?.to_vec();
    Ok(audio_data)

    // hayden_voice_mixer(audio_data).map_err(|e| e.to_string())
}
