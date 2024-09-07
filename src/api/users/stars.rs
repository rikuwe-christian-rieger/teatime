use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Repository, Client};

#[derive(Default, Debug, Serialize, Builder)]
#[build_it(into)]
pub struct ListStarredBuilder {
    #[build_it(skip)]
    #[serde(skip)]
    username: String,

    /// Page number of results to return (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u64>,

    /// Page size of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

impl ListStarredBuilder {
    pub fn new(username: impl ToString) -> Self {
        Self {
            username: username.to_string(),
            ..Default::default()
        }
    }

    /// Sends the request to get the user's stars.
    pub async fn send(&self, client: &Client) -> Result<Vec<Repository>> {
        let req = client
            .get(format!("/users/{}/starred", self.username))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
