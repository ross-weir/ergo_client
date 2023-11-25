pub mod root;
pub mod transactions;
pub mod wallet;

use std::rc::Rc;

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
    root: RootEndpoint,
    wallet: WalletEndpoint,
    transactions: TransactionsEndpoint,
}

impl NodeEndpoint {
    pub fn new(client: Client, url: Url) -> Result<Self, crate::Error> {
        let shared_client = Rc::new(client);
        Ok(Self {
            root: RootEndpoint::new(shared_client.clone(), url.clone())?,
            wallet: WalletEndpoint::new(shared_client.clone(), url.clone())?,
            transactions: TransactionsEndpoint::new(shared_client, url.clone())?,
        })
    }

    pub fn root(&self) -> &RootEndpoint {
        &self.root
    }

    pub fn wallet(&self) -> &WalletEndpoint {
        &self.wallet
    }

    pub fn transactions(&self) -> &TransactionsEndpoint {
        &self.transactions
    }
}
