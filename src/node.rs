pub mod endpoints;
pub mod extensions;

use self::{endpoints::NodeEndpoint, extensions::NodeExtension};
use crate::common::CoreError;
use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Client, Url,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum NodeError {
    #[error("Nodes wallet doesn't hold enough nanoergs, {found} < {requested}")]
    InsufficientFunds { requested: u64, found: u64 },

    #[error("Specified API key is not a valid header value")]
    InvalidApiKey {
        source: InvalidHeaderValue,
        key: String,
    },

    /// Node returned a 4xx or 5xx error code.
    ///
    /// Returns the description of the error returned by the node
    /// in the `detail` field in the JSON response.
    #[error("Node returned error: {0}")]
    BadRequest(String),

    #[error(transparent)]
    Core(#[from] CoreError),
}

/// Error object returned by the nodes API.
#[derive(Debug, Deserialize)]
pub struct NodeApiError {
    /// Status code
    pub error: u32,
    /// Short text that sometimes describes the error, other times its unhelpful
    pub reason: String,
    /// Usually a more helpful string describing what error occurred
    pub detail: String,
}

pub(crate) async fn process_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, NodeError> {
    if response.status().is_success() {
        Ok(response
            .json::<T>()
            .await
            .map_err(CoreError::ResponseDeserialization)?)
    } else {
        let node_err = response
            .json::<NodeApiError>()
            .await
            .map_err(CoreError::ResponseDeserialization)?;
        Err(NodeError::BadRequest(node_err.detail).into())
    }
}

#[derive(Debug, Clone)]
pub struct NodeClient {
    endpoints: NodeEndpoint,
}

impl NodeClient {
    pub fn from_url_str(
        url_str: &str,
        api_key: String,
        timeout: Duration,
    ) -> Result<Self, NodeError> {
        let url = Url::parse(url_str).map_err(CoreError::UrlParse)?;
        let mut headers = HeaderMap::new();
        let mut key_header_val =
            HeaderValue::from_str(&api_key).map_err(|e| NodeError::InvalidApiKey {
                source: e,
                key: api_key,
            })?;
        key_header_val.set_sensitive(true);
        headers.insert("api_key", key_header_val);
        let client = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            // outputs all connection events if `trace` log level is set for `reqwest` crate
            // useful to debug response errors
            .connection_verbose(true)
            .build()
            .map_err(CoreError::BuildClient)?;
        Ok(Self {
            endpoints: NodeEndpoint::new(client, url),
        })
    }

    pub fn endpoints(&self) -> &NodeEndpoint {
        &self.endpoints
    }

    pub fn extensions(&self) -> NodeExtension {
        NodeExtension::new(&self.endpoints)
    }
}
