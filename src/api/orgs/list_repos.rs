use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Repository, Client};

#[derive(Debug, Clone, Builder, Serialize)]
#[build_it(into)]
pub struct ListReposBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    org: String,
    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results.
    limit: Option<i64>,
}

impl ListReposBuilder {
    pub fn new(org: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list an organization's repositories.
    pub async fn send(&self, client: &Client) -> Result<Vec<Repository>> {
        let req = client
            .get(format!("/orgs/{}/repos", self.org))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
