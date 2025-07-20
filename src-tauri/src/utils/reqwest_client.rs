use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ReqwestClent {
    pub client: Arc<Mutex<Client>>,
}

impl ReqwestClent {
    pub fn new() -> Self {
        ReqwestClent {
            client: Arc::new(Mutex::new(Client::new())),
        }
    }

    pub async fn get(
        &self,
        url: &str,
        params: Option<&[(String, String)]>,
    ) -> Result<String, reqwest::Error> {
        let client = self.client.lock().await;
        let request = client.get(url);

        let request = if let Some(params) = params {
            request.query(params)
        } else {
            request
        };

        let response = request.send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn post(
        &self,
        url: &str,
        body: Option<serde_json::Value>,
        params: Option<&[(String, String)]>,
    ) -> Result<String, reqwest::Error> {
        let client = self.client.lock().await;
        let mut request = client.post(url);

        if let Some(params) = params {
            request = request.query(params);
        }

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}
