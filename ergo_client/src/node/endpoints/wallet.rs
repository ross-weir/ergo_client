pub mod boxes;

use self::boxes::BoxesEndpoint;
use crate::Error;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug)]
pub struct WalletEndpoint {
    client: Rc<Client>,
    base_url: Url,
    boxes_endpoint: BoxesEndpoint,
}

impl WalletEndpoint {
    pub fn new(client: Rc<Client>, base_url: Url) -> Result<Self, crate::Error> {
        let mut boxes_url = base_url.clone();
        boxes_url
            .path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("boxes");
        Ok(Self {
            client: client.clone(),
            base_url,
            boxes_endpoint: BoxesEndpoint::new(client, boxes_url),
        })
    }

    pub fn boxes(&self) -> &BoxesEndpoint {
        &self.boxes_endpoint
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

impl WalletEndpoint {
    pub async fn status(&self) -> Result<StatusResponse, Error> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("status");

        self.client
            .get(url.clone())
            .send()
            .await?
            .json()
            .await
            .map_err(|e| Error::ResponseDeserialization {
                url: url.to_string(),
                cause: e,
            })
    }
}
