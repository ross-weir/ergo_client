use ergo_lib::ergotree_ir::serialization::{SigmaParsingError, SigmaSerializationError};

pub mod node;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to build http client")]
    BuildClient(reqwest::Error),

    #[error("Failed to parse url: {0}")]
    UrlParsing(String),

    #[error("Failed to append segment to url")]
    AppendPathSegment,

    #[error("Failed to perform request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to deserialize response from url '{url}' due to: {cause}")]
    ResponseDeserialization { url: String, cause: reqwest::Error },

    #[error("Failed to serialize to bytes")]
    SigmaSerialization(#[from] SigmaSerializationError),

    #[error("Failed to parse bytes")]
    SigmaParsing(#[from] SigmaParsingError),

    #[error("Node error occurred")]
    Node(#[from] node::NodeError),
}
