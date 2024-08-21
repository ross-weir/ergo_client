use ergo_lib::{ergo_chain_types::EcPoint, ergotree_ir::chain::address::NetworkAddress};
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::node::{process_response, NodeError};

#[derive(Debug, Clone)]
pub struct UtilsEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> UtilsEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("utils");
        Ok(Self { client, url })
    }
}

#[derive(Deserialize)]
struct RawToAddressResponse {
    address: NetworkAddress,
}

impl<'a> UtilsEndpoint<'a> {
    pub async fn raw_to_address(&self, pubkey: EcPoint) -> Result<NetworkAddress, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("rawToAddress")
            .push(&pubkey.to_string());
        Ok(process_response::<RawToAddressResponse>(
            self.client.get(url).send().await.map_err(NodeError::Http)?,
        )
        .await?
        .address)
    }
}
