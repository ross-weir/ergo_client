use serde::Serialize;

pub mod root;
pub mod transactions;
pub mod wallet;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePaginationQuery {
    limit: u32,
    offset: u32,
}

impl Default for NodePaginationQuery {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
        }
    }
}
