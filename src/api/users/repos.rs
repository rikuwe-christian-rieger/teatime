use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Repository, Client};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct ListReposBuilder {
    #[build_it(skip)]
    #[serde(skip)]
    username: String,

    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results.
    limit: Option<i64>,
}

impl ListReposBuilder {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            page: None,
            limit: None,
        }
    }

    /// Sends the request to get a user's repositories.
    pub async fn send(&self, client: &Client) -> Result<Vec<Repository>> {
        let req = client
            .get(format!("users/{}/repos", self.username))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
