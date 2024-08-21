pub mod blockchain;
pub mod blocks;
pub mod root;
pub mod scan;
pub mod script;
pub mod transactions;
pub mod utils;
pub mod wallet;

use self::root::RootEndpoint;
use self::script::ScriptEndpoint;
use self::transactions::TransactionsEndpoint;
use self::wallet::WalletEndpoint;
use blockchain::BlockchainEndpoint;
use blocks::BlocksEndpoint;
use reqwest::{Client, Url};
use scan::ScanEndpoint;
use serde::Serialize;
use utils::UtilsEndpoint;

use super::NodeError;

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
    pub fn new(client: Client, url: Url) -> Self {
        Self { client, url }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn root(&self) -> Result<RootEndpoint, NodeError> {
        RootEndpoint::new(&self.client, self.url.clone())
    }

    pub fn blocks(&self) -> Result<BlocksEndpoint, NodeError> {
        BlocksEndpoint::new(&self.client, self.url.clone())
    }

    pub fn wallet(&self) -> Result<WalletEndpoint, NodeError> {
        WalletEndpoint::new(&self.client, self.url.clone())
    }

    pub fn transactions(&self) -> Result<TransactionsEndpoint, NodeError> {
        TransactionsEndpoint::new(&self.client, self.url.clone())
    }

    pub fn script(&self) -> Result<ScriptEndpoint, NodeError> {
        ScriptEndpoint::new(&self.client, self.url.clone())
    }

    pub fn utils(&self) -> Result<UtilsEndpoint, NodeError> {
        UtilsEndpoint::new(&self.client, self.url.clone())
    }

    pub fn blockchain(&self) -> Result<BlockchainEndpoint, NodeError> {
        BlockchainEndpoint::new(&self.client, self.url.clone())
    }

    pub fn scan(&self) -> Result<ScanEndpoint, NodeError> {
        ScanEndpoint::new(&self.client, self.url.clone())
    }
}
