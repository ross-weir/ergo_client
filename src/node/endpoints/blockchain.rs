use ergo_lib::{
    chain::transaction::{DataInput, TxId},
    ergo_chain_types::BlockId,
    ergotree_ir::chain::{
        ergo_box::{BoxId, ErgoBox},
        token::TokenId,
    },
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use crate::node::{process_response, NodeError};

#[derive(Serialize)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexQuery {
    pub offset: u64,
    pub limit: u32,
    pub sort_direction: SortDirection,
    pub include_unconfirmed: bool,
}

#[derive(Deserialize, Debug)]
pub struct IndexedBox {
    #[serde(flatten)]
    pub ergo_box: ErgoBox,
    #[serde(rename = "globalIndex")]
    pub global_index: u64,
    #[serde(rename = "spentTransactionId")]
    pub spent_transaction_id: Option<TxId>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedTransaction {
    pub id: TxId,
    pub block_id: BlockId,
    pub inputs: Vec<ErgoBox>,
    pub outputs: Vec<ErgoBox>,
    pub data_inputs: Vec<DataInput>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedHeight {
    pub indexed_height: u32,
    pub full_height: u32,
}

#[derive(Debug, Clone)]
pub struct BlockchainEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> BlockchainEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("blockchain");
        Ok(Self { client, url })
    }
}

impl<'a> BlockchainEndpoint<'a> {
    pub async fn indexed_height(&self) -> Result<IndexedHeight, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("indexedHeight");
        process_response::<IndexedHeight>(
            self.client.get(url).send().await.map_err(NodeError::Http)?,
        )
        .await
    }

    pub async fn unspent_by_address(
        &self,
        address: &str,
        query: IndexQuery,
    ) -> Result<Vec<IndexedBox>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["box", "unspent", "byAddress"]);
        process_response(
            self.client
                .post(url)
                .query(&query)
                .json(address)
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await
    }

    pub async fn get_transaction_by_id(
        &self,
        tx_id: &TxId,
    ) -> Result<IndexedTransaction, NodeError> {
        let mut url = self.url.clone();
        let tx_id = tx_id.to_string();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["transaction", "byId", &tx_id]);
        process_response(self.client.get(url).send().await.map_err(NodeError::Http)?).await
    }

    pub async fn get_box_by_id(&self, box_id: &BoxId) -> Result<IndexedBox, NodeError> {
        let mut url = self.url.clone();
        let box_id = box_id.to_string();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["box", "byId", &box_id]);
        process_response(self.client.get(url).send().await.map_err(NodeError::Http)?).await
    }

    pub async fn get_unspent_boxes_by_token_id(
        &self,
        token_id: &str,
        index_query: IndexQuery,
    ) -> Result<Vec<IndexedBox>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["box", "unspent", "byTokenId", &token_id]);
        Ok(process_response(
            self.client
                .get(url)
                .query(&index_query)
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await?)
    }
}
