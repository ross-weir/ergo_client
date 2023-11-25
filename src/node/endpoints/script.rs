use crate::Error;
use reqwest::{Client, Url};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct ScriptEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> ScriptEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, Error> {
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("script");
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize)]
struct AddressToTreeResponse {
    tree: String,
}

impl<'a> ScriptEndpoint<'a> {
    pub async fn address_to_tree(&self, network_address: &str) -> Result<String, Error> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("addressToTree")
            .push(network_address);
        Ok(self
            .client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?
            .json::<AddressToTreeResponse>()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: url.to_string(),
                source: e,
            })?
            .tree)
    }
}

#[derive(Debug, Deserialize)]
struct P2sAddressResponse {
    address: String,
}

impl<'a> ScriptEndpoint<'a> {
    /// Compiles the provided ErgoScript source code to a network encoded address.
    pub async fn p2s_address(&self, source: &str) -> Result<String, Error> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("p2sAddress");
        let body = json!({
            "source": source
        });
        Ok(self
            .client
            .post(url.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<P2sAddressResponse>()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: url.to_string(),
                source: e,
            })?
            .address)
    }
}
