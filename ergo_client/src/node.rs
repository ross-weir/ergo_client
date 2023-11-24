pub mod endpoints;

use self::endpoints::wallet::WalletEndpoint;
use crate::Error;
use reqwest::{header::HeaderMap, Client, Url};
use serde::Deserialize;
use std::{rc::Rc, time::Duration};

#[derive(Debug)]
pub struct NodeClient {
    client: Rc<Client>,
    base_url: Url,
    wallet: WalletEndpoint,
}

impl NodeClient {
    pub fn from_url_str(url_str: &str, api_key: String, timeout: Duration) -> Result<Self, Error> {
        let base_url = Url::parse(url_str).map_err(|e| Error::UrlParsing(e.to_string()))?;
        let mut headers = HeaderMap::new();
        headers.insert("api_key", api_key.clone().try_into()?);
        let client = Rc::new(
            Client::builder()
                .default_headers(headers)
                .timeout(timeout)
                .build()
                .unwrap(),
        );
        let wallet_url = base_url
            .join("wallet")
            .map_err(|e| Error::UrlParsing(e.to_string()))?;
        let wallet = WalletEndpoint::new(client.clone(), wallet_url)?;
        Ok(Self {
            client,
            base_url,
            wallet,
        })
    }

    pub fn base_url(&self) -> Url {
        self.base_url.clone()
    }

    pub fn wallet(&self) -> &WalletEndpoint {
        &self.wallet
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub difficulty: u64,
    pub full_height: i32,
}

impl NodeClient {
    pub async fn info(&self) -> Result<InfoResponse, Error> {
        let url = self
            .base_url
            .join("info")
            .map_err(|e| Error::UrlParsing(e.to_string()))?;
        self.client
            .get(url.clone())
            .send()
            .await?
            .json()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: url.to_string(),
                cause: e,
            })
    }
}
