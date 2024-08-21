use std::ops::Range;

use ergo_lib::{
    chain::transaction::Transaction,
    ergo_chain_types::{BlockId, Header},
};
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::node::{process_response, NodeError};

#[derive(Debug, Clone)]
pub struct BlocksEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> BlocksEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("blocks");
        Ok(Self { client, url })
    }

    /// Get header id at given height (/blocks/at/{blockHeight} endpoint)
    pub async fn block_at_height(&self, block_height: u32) -> Result<Option<BlockId>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["at", &format!("{block_height}")]);
        Ok(process_response::<Vec<BlockId>>(
            self.client.get(url).send().await.map_err(NodeError::Http)?,
        )
        .await?
        .get(0)
        .cloned())
    }

    pub async fn chain_slice(&self, range: Range<u32>) -> Result<Vec<Header>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("chainSlice");
        Ok(process_response(
            self.client
                .get(url)
                .query(&[("fromHeight", range.start), ("toHeight", range.end)])
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await?)
    }

    pub async fn transactions(&self, block_id: &BlockId) -> Result<Vec<Transaction>, NodeError> {
        #[derive(Deserialize)]
        struct BlockTransactions {
            transactions: Vec<Transaction>,
        }
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&[&block_id.to_string(), "transactions"]);
        Ok(process_response::<BlockTransactions>(
            self.client.get(url).send().await.map_err(NodeError::Http)?,
        )
        .await?
        .transactions)
    }
}
