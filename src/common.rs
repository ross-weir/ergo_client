#[derive(thiserror::Error, Debug)]
pub enum CoreError {
    #[error("Failed to build HTTP client")]
    BuildClient(#[source] reqwest::Error),

    #[error("Failed to parse URL")]
    UrlParse(#[from] url::ParseError),

    #[error("Failed to append segment to url")]
    AppendPathSegment,

    #[error("HTTP request failed")]
    Http(#[from] reqwest::Error),

    #[error("Failed to deserialize response")]
    ResponseDeserialization(#[source] reqwest::Error),
}
