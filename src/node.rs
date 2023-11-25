pub mod endpoints;
pub mod extensions;

use self::{endpoints::NodeEndpoint, extensions::NodeExtension};
use crate::Error;
use reqwest::{header::HeaderMap, Client, Url};
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    #[error("Nodes wallet doesn't hold enough nanoergs, {found} < {requested}")]
    InsufficientFunds { requested: u64, found: u64 },
}

#[derive(Debug)]
pub struct NodeClient {
    url: Url,
    endpoints: NodeEndpoint,
    extensions: NodeExtension,
}

impl NodeClient {
    pub fn from_url_str(url_str: &str, api_key: String, timeout: Duration) -> Result<Self, Error> {
        let url = Url::parse(url_str).map_err(|e| Error::UrlParsing(e.to_string()))?;
        let mut headers = HeaderMap::new();
        headers.insert("api_key", api_key.clone().try_into()?);
        let client = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .map_err(|e| Error::BuildClient(e))?;
        let endpoints = NodeEndpoint::new(client, url.clone())?;
        Ok(Self {
            url,
            extensions: NodeExtension::new(endpoints.clone()),
            endpoints,
        })
    }

    pub fn base_url(&self) -> Url {
        self.url.clone()
    }

    pub fn endpoints(&self) -> &NodeEndpoint {
        &self.endpoints
    }

    pub fn extensions(&self) -> &NodeExtension {
        &self.extensions
    }
}
