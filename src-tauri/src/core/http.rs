
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};

use crate::core::errors::{FeatherError, FeatherResult};

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> FeatherResult<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("FeatherLauncher/0.1.0"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|error| FeatherError::Internal(error.to_string()))?;

        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
