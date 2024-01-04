use crate::common::CoreError;
use crate::node::{process_response, NodeError};
use ergo_lib::chain::transaction::{unsigned::UnsignedTransaction, Transaction};
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TransactionEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> TransactionEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("transaction");
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRequest {
    tx: UnsignedTransaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs_raw: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_inputs_raw: Option<Vec<String>>,
}

impl<'a> TransactionEndpoint<'a> {
    pub async fn sign(
        &self,
        unsigned_tx: UnsignedTransaction,
        inputs: Option<Vec<ErgoBox>>,
        data_inputs: Option<Vec<ErgoBox>>,
    ) -> Result<Transaction, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("sign");
        let body = SignRequest {
            tx: unsigned_tx,
            inputs_raw: inputs
                .map(|boxes| boxes.iter().map(|b| String::from(b.box_id())).collect()),
            data_inputs_raw: data_inputs
                .map(|boxes| boxes.iter().map(|b| String::from(b.box_id())).collect()),
        };
        process_response(
            self.client
                .post(url)
                .json(&body)
                .send()
                .await
                .map_err(CoreError::Http)?,
        )
        .await
    }
}
