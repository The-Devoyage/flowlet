use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::util::FlowletResult;

pub struct ApiClient {
    base_url: Url,
    client: Client,
}

#[derive(Debug, Error)]
enum ApiClientError {
    #[error("Failed to parse Base Server URL.")]
    UrlParseError,

    #[error("Failed to POST: {0}")]
    PostError(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmptyData;

impl ApiClient {
    pub fn new(base_url: &str) -> FlowletResult<Self> {
        Ok(Self {
            base_url: Url::parse(base_url).map_err(|e| {
                log::error!("{:?}", e);
                ApiClientError::UrlParseError
            })?,
            client: Client::new(),
        })
    }

    pub async fn post<T, K>(&self, path: &str, body: &T) -> FlowletResult<ApiResponse<K>>
    where
        T: Serialize,
        K: for<'de> Deserialize<'de>,
    {
        let url = self.base_url.join(path).unwrap();
        let response = self.client.post(url).json(body).send().await.map_err(|e| {
            log::error!("{:?}", e);
            ApiClientError::PostError(e.to_string())
        })?;
        let status = response.status();

        if !status.is_success() {
            log::error!("Network request failed with status: {}", status);
            return Err(Box::new(ApiClientError::PostError(format!(
                "Network request failed with status: {}",
                status
            ))));
        }

        let body = response.text().await.unwrap_or_default();
        let parsed: ApiResponse<K> = serde_json::from_str(&body).map_err(|e| {
            log::error!("{:?}", e);
            ApiClientError::PostError("Failed to serialize json.".to_string())
        })?;

        Ok(parsed)
    }
}
