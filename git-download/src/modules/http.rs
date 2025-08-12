use anyhow::Result;
use reqwest::{Client, header::HeaderMap};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn make_request<T>(self, api: &str, headers: &HeaderMap) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(api)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|error| format!("Request failed: {}", error))?;
        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<T>()
                .await
                .map_err(|error| format!("Failed to parse response: {}", error)),
            status => Err(format!("Request failed with status: {}", status)),
        }
    }
}
