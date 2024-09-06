use build_it::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    model::orgs::{Organization, Visibility},
    Client,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[build_it(into)]
pub struct EditOrgBuilder {
    #[serde(skip)]
    #[skip]
    name: String,
    description: Option<String>,
    email: Option<String>,
    full_name: Option<String>,
    location: Option<String>,
    repo_admin_change_team_access: Option<bool>,
    visibility: Option<Visibility>,
    website: Option<String>,
}

impl EditOrgBuilder {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            email: None,
            full_name: None,
            location: None,
            repo_admin_change_team_access: None,
            visibility: None,
            website: None,
        }
    }
    pub async fn send(&self, client: &Client) -> Result<Organization> {
        let req = client
            .patch(format!("orgs/{}", self.name))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
