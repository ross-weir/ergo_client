pub mod boxes;
pub mod transaction;

use self::{boxes::BoxesEndpoint, transaction::TransactionEndpoint};
use crate::Error;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WalletEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> WalletEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, crate::Error> {
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("wallet");
        Ok(Self { client, url })
    }

    pub fn transaction(&self) -> Result<TransactionEndpoint, Error> {
        TransactionEndpoint::new(&self.client, self.url.clone())
    }

    pub fn boxes(&self) -> Result<BoxesEndpoint, Error> {
        BoxesEndpoint::new(&self.client, self.url.clone())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    pub is_initialized: bool,
    pub is_unlocked: bool,
    pub change_address: String,
    pub wallet_height: i64,
    pub error: String,
}

impl<'a> WalletEndpoint<'a> {
    pub async fn status(&self) -> Result<StatusResponse, Error> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("status");

        self.client
            .get(url.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: url.to_string(),
                source: e,
            })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockRequest {
    password: String,
}

impl<'a> WalletEndpoint<'a> {
    pub async fn unlock(&self, password: String) -> Result<(), Error> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("unlock");
        let body = UnlockRequest { password };

        self.client
            .post(url.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
