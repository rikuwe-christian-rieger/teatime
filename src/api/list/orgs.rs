use crate::error::Result;
use crate::model::orgs::Organization;
use build_it::Builder;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct ListOrgsBuilder {}

impl ListOrgsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Organization>> {
        let req = client.get("orgs").build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
