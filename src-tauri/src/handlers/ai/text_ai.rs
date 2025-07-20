use crate::{handlers::ai::urls::v1, utils::reqwest_client::ReqwestClent};
use reqwest::Error;
use serde::Serialize;

#[derive(Serialize)]
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

impl TextAi {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        vec![
            ("prompt".to_string(), self.prompt.clone()),
            ("model".to_string(), self.model.clone()),
            ("seed".to_string(), self.seed.to_string()),
            ("temperature".to_string(), self.temperature.to_string()),
            ("top_p".to_string(), self.top_p.to_string()),
            (
                "presence_penalty".to_string(),
                self.presence_penalty.to_string(),
            ),
            ("json".to_string(), self.json.to_string()),
            ("system".to_string(), self.system.clone()),
            ("stream".to_string(), self.stream.to_string()),
            ("private".to_string(), self.private.to_string()),
            ("referrer".to_string(), self.referrer.clone()),
        ]
    }
}

// pub async fn generate_text(
//     client: &ReqwestClent,
//     options: TextAi,
// ) -> Result<String, reqwest::Error> {
//     let url = format!("{}", v1::BASE);
//     let query_params = options.to_query_params();
//
//     let response = client.get(&url, &options);
//
//     let body = response.text().await?;
//     Ok(body)
// }
