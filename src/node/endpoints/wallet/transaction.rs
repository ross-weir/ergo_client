use crate::Error;
use ergo_lib::chain::transaction::{unsigned::UnsignedTransaction, Transaction};
use ergo_lib::ergo_chain_types::Base16EncodedBytes;
use ergo_lib::ergotree_ir::chain::ergo_box::ErgoBox;
use ergo_lib::ergotree_ir::serialization::{SigmaSerializable, SigmaSerializationError};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug)]
pub struct TransactionEndpoint {
    client: Rc<Client>,
    url: Url,
}

impl TransactionEndpoint {
    pub fn new(client: Rc<Client>, mut url: Url) -> Result<Self, Error> {
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("transaction");
        Ok(Self { client, url })
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRequest {
    tx: UnsignedTransaction,
    inputs_raw: Vec<String>,
    data_inputs_raw: Vec<String>,
}

impl TransactionEndpoint {
    pub fn to_raw_boxes(&self, boxes: Vec<ErgoBox>) -> Result<Vec<String>, Error> {
        Ok(boxes
            .iter()
            .map(|b| {
                b.sigma_serialize_bytes()
                    .map(|b| Base16EncodedBytes::new(&b).into())
            })
            .collect::<Result<Vec<String>, SigmaSerializationError>>()?)
    }

    pub async fn sign(
        &self,
        unsigned_tx: UnsignedTransaction,
        inputs: Vec<ErgoBox>,
        data_inputs: Option<Vec<ErgoBox>>,
    ) -> Result<Transaction, Error> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::AppendPathSegment)?
            .push("sign");
        let body = SignRequest {
            tx: unsigned_tx,
            inputs_raw: self.to_raw_boxes(inputs)?,
            data_inputs_raw: self.to_raw_boxes(data_inputs.unwrap_or_default())?,
        };

        self.client
            .post(url.clone())
            .json(&body)
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
