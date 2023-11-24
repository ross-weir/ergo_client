use crate::Error;
use ergo_lib::chain::transaction::Transaction;
use reqwest::{Client, Url};
use std::rc::Rc;

#[derive(Debug)]
pub struct TransactionsEndpoint {
    client: Rc<Client>,
    url: Url,
}

impl TransactionsEndpoint {
    pub fn new(client: Rc<Client>, mut url: Url) -> Result<Self, Error> {
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("transactions");
        Ok(Self { client, url })
    }
}

impl TransactionsEndpoint {
    /// POST /transactions
    /// Node returns the transaction id string directly, not inside an object or array
    pub async fn submit(&self, tx: Transaction) -> Result<String, Error> {
        self.client
            .post(self.url.clone())
            .json(&tx)
            .send()
            .await?
            .json()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: self.url.to_string(),
                cause: e,
            })
    }
}
