use crate::{error::Result, model::orgs::Organization, Client};

pub struct GetOrgBuilder {
    name: String,
}

impl GetOrgBuilder {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    /// Send the request to get an [Organization].
    pub async fn send(&self, client: &Client) -> Result<Organization> {
        let req = client.get(&format!("orgs/{}", self.name)).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
