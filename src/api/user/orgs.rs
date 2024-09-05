use build_it::Builder;
use teatime_macros::QueryParams;

use crate::{model::orgs::Organization, Client};

#[derive(Debug, Default, Builder, QueryParams)]
pub struct Orgs {
    page: Option<i64>,
    limit: Option<i64>,
}

impl Orgs {
    pub fn new() -> Self {
        Self::default()
    }
    /// Send the request to get the current user's organizations.
    pub async fn send(&self, client: &Client) -> crate::Result<Vec<Organization>> {
        let mut req = client.get("user/orgs").build()?;
        self.append_query_params(&mut req);
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
