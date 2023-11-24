use crate::Error;
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug)]
pub struct BoxesEndpoint {
    client: Rc<Client>,
    base_url: Url,
}

impl BoxesEndpoint {
    pub fn new(client: Rc<Client>, base_url: Url) -> Self {
        Self { client, base_url }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

impl BoxesEndpoint {
    pub async fn unspent(
        &self,
        query: Option<UnspentQuery>,
    ) -> Result<Vec<UnspentResponseEntry>, Error> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("unspent");

        self.client
            .get(url.clone())
            .query(&query.unwrap_or_default())
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
