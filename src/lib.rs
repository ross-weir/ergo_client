use ergo_lib::ergotree_ir::serialization::{SigmaParsingError, SigmaSerializationError};

pub mod node;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to build HTTP client")]
    BuildClient(reqwest::Error),

    #[error("Failed to parse URL")]
    UrlParse(#[from] url::ParseError),

    #[error("Failed to append segment to url")]
    AppendPathSegment,

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("Failed to deserialize response")]
    ResponseDeserialization(#[source] reqwest::Error),

    #[error("Failed to serialize to bytes")]
    SigmaSerialization(#[from] SigmaSerializationError),

    #[error("Failed to parse bytes")]
    SigmaParsing(#[from] SigmaParsingError),

    #[error("Node error occurred")]
    Node(#[from] node::NodeError),
}
