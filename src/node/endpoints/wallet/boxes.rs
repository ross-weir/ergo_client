use crate::node::{process_response, NodeError};
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BoxesEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> BoxesEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("boxes");
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnspentResponseEntry {
    pub confirmations_num: u32,
    pub address: String,
    pub creation_transaction: String,
    #[serde(rename = "box")]
    pub ergo_box: ErgoBox,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnspentQuery {
    pub min_confirmations: i32,
    pub max_confirmations: i32,
    pub min_inclusion_height: i32,
    pub max_inclusion_height: i32,
}

impl Default for UnspentQuery {
    fn default() -> Self {
        // -1 means unlimited
        // these are the defaults in the api docs
        Self {
            min_confirmations: 0,
            max_confirmations: -1,
            min_inclusion_height: 0,
            max_inclusion_height: -1,
        }
    }
}

impl<'a> BoxesEndpoint<'a> {
    pub async fn unspent(
        &self,
        query: Option<UnspentQuery>,
    ) -> Result<Vec<UnspentResponseEntry>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("unspent");

        process_response(
            self.client
                .get(url)
                .query(&query.unwrap_or_default())
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await
    }
}
