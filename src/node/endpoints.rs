pub mod root;
pub mod transactions;
pub mod wallet;

use reqwest::{Client, Url};
use serde::Serialize;

use self::{root::RootEndpoint, transactions::TransactionsEndpoint, wallet::WalletEndpoint};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePaginationQuery {
    limit: u32,
    offset: u32,
}

impl Default for NodePaginationQuery {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeEndpoint {
    client: Client,
    url: Url,
}

impl NodeEndpoint {
    pub fn new(client: Client, url: Url) -> Result<Self, crate::Error> {
        Ok(Self { client, url })
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn root(&self) -> Result<RootEndpoint, crate::Error> {
        RootEndpoint::new(&self.client, self.url.clone())
    }

    pub fn wallet(&self) -> Result<WalletEndpoint, crate::Error> {
        WalletEndpoint::new(&self.client, self.url.clone())
    }

    pub fn transactions(&self) -> Result<TransactionsEndpoint, crate::Error> {
        TransactionsEndpoint::new(&self.client, self.url.clone())
    }
}
