use std::borrow::Cow;

use ergo_lib::{
    chain::transaction::TxId,
    ergo_chain_types::Base16DecodedBytes,
    ergotree_ir::{
        chain::{
            ergo_box::{ErgoBox, RegisterId},
            token::TokenId,
        },
        mir::constant::Constant,
        serialization::SigmaSerializable,
    },
};
use reqwest::Client;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use url::Url;

use crate::node::{process_response, NodeError};

fn serialize_constant<S: Serializer>(constant: &Constant, ser: S) -> Result<S::Ok, S::Error> {
    Base16DecodedBytes(constant.sigma_serialize_bytes().unwrap()).serialize(ser)
}

fn deserialize_constant<'de, D: Deserializer<'de>>(d: D) -> Result<Constant, D::Error> {
    let bytes = Base16DecodedBytes::try_from(String::deserialize(d)?).map_err(de::Error::custom)?;
    Constant::sigma_parse_bytes(&bytes.0).map_err(de::Error::custom)
}
fn serialize_register_id<S: Serializer>(
    register_id: &Option<RegisterId>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    match register_id.as_ref() {
        Some(id) => ser.serialize_str(&format!("{id}")),
        None => ser.serialize_none(),
    }
}

fn deserialize_register_id<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<RegisterId>, D::Error> {
    let register_id = Option::<&str>::deserialize(d)?;
    match register_id {
        Some(id) => match id
            .split_at_checked(1)
            .ok_or(de::Error::custom("Failed to parse "))?
        {
            ("R", id) => RegisterId::try_from(id.parse::<i8>().map_err(de::Error::custom)?)
                .map_err(de::Error::custom)
                .map(Some),
            _ => Err(de::Error::custom("Failed to parse register id")),
        },
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "predicate")]
pub enum TrackingRule {
    #[serde(rename = "equals")]
    Equals {
        #[serde(
            serialize_with = "serialize_register_id",
            deserialize_with = "deserialize_register_id",
            skip_serializing_if = "Option::is_none",
            default
        )]
        register: Option<RegisterId>,
        #[serde(
            serialize_with = "serialize_constant",
            deserialize_with = "deserialize_constant"
        )]
        value: Constant,
    },
    #[serde(rename = "contains")]
    Contains {
        #[serde(
            serialize_with = "serialize_register_id",
            deserialize_with = "deserialize_register_id",
            skip_serializing_if = "Option::is_none",
            default
        )]
        register: Option<RegisterId>,
        #[serde(
            serialize_with = "serialize_constant",
            deserialize_with = "deserialize_constant"
        )]
        value: Constant,
    },
    #[serde(rename = "containsAsset")]
    ContainsAsset {
        #[serde(rename = "assetId")]
        asset_id: TokenId,
    },
    #[serde(rename = "and")]
    And { args: Vec<TrackingRule> },
    #[serde(rename = "or")]
    Or { args: Vec<TrackingRule> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Scan<'a> {
    pub scan_name: Cow<'a, str>,
    pub wallet_interaction: Cow<'a, str>,
    pub tracking_rule: TrackingRule,
    pub remove_offchain: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredScan<'a> {
    pub scan_id: u32,
    #[serde(flatten)]
    pub scan: Scan<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScanQuery {
    pub min_confirmations: i32,
    pub max_confirmations: i32,
    pub min_inclusion_height: i32,
    pub max_inclusion_height: i32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScanBox {
    #[serde(rename = "confirmationsNum")]
    pub confirmations: u32,
    #[serde(rename = "spendingTransaction")]
    pub spending_transaction: Option<TxId>,
    #[serde(rename = "spendingHeight")]
    pub spending_height: Option<u32>,
    #[serde(rename = "inclusionHeight")]
    pub inclusion_height: Option<u32>,
    #[serde(rename = "box")]
    pub ergo_box: ErgoBox,
}

#[derive(Debug, Clone)]
pub struct ScanEndpoint<'a> {
    client: &'a Client,
    url: Url,
}

impl<'a> ScanEndpoint<'a> {
    pub fn new(client: &'a Client, mut url: Url) -> Result<Self, NodeError> {
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("scan");
        Ok(Self { client, url })
    }

    pub async fn register<'s>(&self, scan: &Scan<'s>) -> Result<u32, NodeError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ScanResponse {
            scan_id: u32,
        }
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("register");
        Ok(process_response::<ScanResponse>(
            self.client
                .post(url)
                .json(scan)
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await?
        .scan_id)
    }

    pub async fn deregister(&self, scan_id: u32) -> Result<(), NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("deregister");
        self.client
            .post(url)
            .json(&json!( { "scanId": scan_id}))
            .send()
            .await
            .map_err(NodeError::Http)?;
        Ok(())
    }

    pub async fn list_all(&self) -> Result<Vec<RegisteredScan<'static>>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .push("listAll");
        process_response(self.client.get(url).send().await.map_err(NodeError::Http)?).await
    }

    pub async fn unspent_boxes(
        &self,
        scan_id: u32,
        query: &ScanQuery,
    ) -> Result<Vec<ScanBox>, NodeError> {
        let mut url = self.url.clone();
        url.path_segments_mut()
            .map_err(|_| NodeError::BaseUrl)?
            .extend(&["unspentBoxes", &format!("{scan_id}")]);
        process_response(
            self.client
                .get(url)
                .query(query)
                .send()
                .await
                .map_err(NodeError::Http)?,
        )
        .await
    }
}
