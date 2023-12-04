use crate::{node::process_response, Error};
use reqwest::{Client, Url};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct RootEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> RootEndpoint<'a> {
    pub fn new(client: &'a Client, url: Url) -> Result<Self, Error> {
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoResponse {
    pub difficulty: u64,
    pub full_height: i32,
}

impl<'a> RootEndpoint<'a> {
    pub async fn info(&self) -> Result<InfoResponse, Error> {
        let url = self.url.join("info")?;
        process_response(self.client.get(url).send().await?).await
    }
}
