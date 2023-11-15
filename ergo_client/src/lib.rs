pub mod node;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse url: {0}")]
    UrlParsing(String),

    #[error("Failed to perform request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to deserialize response from url '{url}' due to: {cause}")]
    ResponseDeserialization { url: String, cause: reqwest::Error },
}
