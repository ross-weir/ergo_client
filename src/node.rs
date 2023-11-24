pub mod endpoints;

use self::endpoints::{
    root::RootEndpoint, transactions::TransactionsEndpoint, wallet::WalletEndpoint,
};
use crate::Error;
use reqwest::{header::HeaderMap, Client, Url};
use std::{rc::Rc, time::Duration};

#[derive(Debug)]
pub struct NodeClient {
    url: Url,
    root: RootEndpoint,
    wallet: WalletEndpoint,
    transactions: TransactionsEndpoint,
}

impl NodeClient {
    pub fn from_url_str(url_str: &str, api_key: String, timeout: Duration) -> Result<Self, Error> {
        let url = Url::parse(url_str).map_err(|e| Error::UrlParsing(e.to_string()))?;
        let mut headers = HeaderMap::new();
        headers.insert("api_key", api_key.clone().try_into()?);
        let client = Rc::new(
            Client::builder()
                .default_headers(headers)
                .timeout(timeout)
                .build()
                .map_err(|e| Error::BuildClient(e))?,
        );
        Ok(Self {
            url: url.clone(),
            root: RootEndpoint::new(client.clone(), url.clone())?,
            wallet: WalletEndpoint::new(client.clone(), url.clone())?,
            transactions: TransactionsEndpoint::new(client, url.clone())?,
        })
    }

    pub fn base_url(&self) -> Url {
        self.url.clone()
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
