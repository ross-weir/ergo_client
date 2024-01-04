use crate::{
    common::CoreError,
    node::{process_response, NodeError},
};
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct ScriptEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> ScriptEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("script");
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize)]
struct AddressToTreeResponse {
    tree: String,
}

impl<'a> ScriptEndpoint<'a> {
    pub async fn address_to_tree(&self, network_address: &str) -> Result<String, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("addressToTree")
            .push(network_address);
        Ok(process_response::<AddressToTreeResponse>(
            self.client.get(url).send().await.map_err(CoreError::Http)?,
        )
        .await?
        .tree)
    }
}

#[derive(Debug, Deserialize)]
struct P2sAddressResponse {
    address: String,
}

impl<'a> ScriptEndpoint<'a> {
    /// Compiles the provided ErgoScript source code to a network encoded address.
    pub async fn p2s_address(&self, source: &str) -> Result<String, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("p2sAddress");
        let body = json!({
            "source": source
        });
        Ok(process_response::<P2sAddressResponse>(
            self.client
                .post(url)
                .json(&body)
                .send()
                .await
                .map_err(CoreError::Http)?,
        )
        .await?
        .address)
    }
}
