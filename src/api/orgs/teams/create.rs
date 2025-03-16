use std::collections::BTreeMap;

use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::team::Team, Client};

/// Represents the options for creating a new user.
/// The only required field is `email` and `username`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct CreateTeamBuilder {
    #[build_it(skip)]
    #[skip]
    #[serde(skip)]
    pub org: String,
    #[build_it(skip)]
    /// Name of the label
    pub name: String,
    /// Description of the label
    pub description: Option<String>,
    /// Permissions of team
    pub permission: Option<String>,
    /// Permission Units of the Team
    pub units: Option<Vec<String>>,
    /// Permission Units of the Team
    pub units_map: Option<BTreeMap<String, String>>,
    /// Whether team is for all repos
    pub includes_all_repositories: Option<bool>,
    /// Whether team is allowed to create repos
    pub can_create_org_repo: Option<bool>,
}

impl CreateTeamBuilder {
    pub fn new(org: impl ToString, name: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            name: name.to_string(),
            description: None,
            includes_all_repositories: None,
            permission: None,
            units: None,
            units_map: None,
            can_create_org_repo: None,
        }
    }

    /// Send the request to create the Team.
    /// This will return the created [Team].
    pub async fn send(&self, client: &Client) -> Result<Team> {
        let org = &self.org;
        let req = client
            .post(format!("orgs/{org}/teams"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
