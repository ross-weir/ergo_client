use reqwest::Url;
use serde::Deserialize;

use crate::Error;

pub mod endpoints;

// TODO:
// - request timeout config
// - user agent header config

#[derive(Debug)]
pub struct NodeClient {
    client: reqwest::Client,
    #[cfg(feature = "blocking")]
    client_blocking: reqwest::blocking::Client,
    base_url: Url,
    api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct InfoResponse {
    pub difficulty: u64,
}

impl NodeClient {
    pub fn from_url_str(url_str: &str, api_key: String) -> Result<Self, Error> {
        let base_url = Url::parse(url_str).map_err(|e| Error::UrlParsing(e.to_string()))?;

        Ok(Self {
            client: Default::default(),
            #[cfg(feature = "blocking")]
            client_blocking: Default::default(),
            base_url,
            api_key,
        })
    }

    #[cfg(feature = "blocking")]
    pub fn info_blocking(&self) -> Result<InfoResponse, Error> {
        let endpoint = self
            .base_url
            .join("/info")
            .map_err(|e| Error::UrlParsing(e.to_string()))?
            .to_string();

        self.client_blocking
            .get(&endpoint)
            .send()?
            .json()
            .map_err(|e| Error::ResponseDeserialization {
                url: endpoint,
                cause: e,
            })
    }

    pub async fn info(&self) -> Result<InfoResponse, Error> {
        let endpoint = self
            .base_url
            .join("/info")
            .map_err(|e| Error::UrlParsing(e.to_string()))?
            .to_string();

        self.client
            .get(&endpoint)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: endpoint,
                cause: e,
            })
    }
}
