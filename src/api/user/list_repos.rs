use build_it::Builder;
use serde::Serialize;
use teatime_macros::QueryParams;

use crate::error::Result;
use crate::model::repos::Repository;

#[derive(Debug, Clone, Serialize, Builder, QueryParams)]
pub struct ListReposBuilder {
    /// Page number of results to return (1-based)
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

impl ListReposBuilder {
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
        }
    }

    /// Send the request to list repositories.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Repository>> {
        let req = client.get("user/repos").build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl Default for ListReposBuilder {
    fn default() -> Self {
        Self::new()
    }
}
