use build_it::Builder;
use serde::Serialize;

use crate::{model::orgs::Organization, Client};

#[derive(Debug, Default, Builder, Serialize)]
#[build_it(into)]
pub struct Orgs {
    #[serde(skip)]
    #[build_it(skip)]
    username: String,
    page: Option<i64>,
    limit: Option<i64>,
}

impl Orgs {
    pub fn new(username: impl ToString) -> Self {
        Self {
            page: None,
            limit: None,
            username: username.to_string(),
        }
    }
    /// Send the request to get the user's organizations.
    pub async fn send(&self, client: &Client) -> crate::Result<Vec<Organization>> {
        let username = &self.username;
        let req = client
            .get(format!("users/{username}/orgs"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
