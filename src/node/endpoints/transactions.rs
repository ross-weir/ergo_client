use crate::{
    common::CoreError,
    node::{process_response, NodeError},
};
use ergo_lib::chain::transaction::Transaction;
use reqwest::{Client, Url};

#[derive(Debug, Clone)]
pub struct TransactionsEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> TransactionsEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| CoreError::AppendPathSegment)?
            .push("transactions");
        Ok(Self { client, url })
    }
}

impl<'a> TransactionsEndpoint<'a> {
    /// POST /transactions
    /// Node returns the transaction id string directly, not inside an object or array
    pub async fn submit(&self, tx: &Transaction) -> Result<String, NodeError> {
        process_response(
            self.client
                .post(self.url.clone())
                .json(&tx)
                .send()
                .await
                .map_err(CoreError::Http)?,
        )
        .await
    }
}
