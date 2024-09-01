use build_it::Builder;
use serde::Serialize;

use crate::{model::repos::Repository, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[serde(default)]
pub struct MigrateRepoBuilder {
    #[skip]
    clone_addr: String,
    #[skip]
    repo_name: String,
    auth_password: Option<String>,
    auth_token: Option<String>,
    auth_username: Option<String>,
    description: Option<String>,
    issues: Option<bool>,
    labels: Option<bool>,
    lfs: Option<bool>,
    lfs_endpoint: Option<String>,
    milestones: Option<bool>,
    mirror: Option<bool>,
    mirror_interval: Option<String>,
    private: Option<bool>,
    pull_requests: Option<bool>,
    releases: Option<bool>,
    repo_owner: Option<String>,
    service: Option<String>,
    wiki: Option<bool>,
}

impl MigrateRepoBuilder {
    pub fn new(clone_addr: impl ToString, repo_name: impl ToString) -> Self {
        Self {
            clone_addr: clone_addr.to_string(),
            repo_name: repo_name.to_string(),
            auth_password: None,
            auth_token: None,
            auth_username: None,
            description: None,
            issues: None,
            labels: None,
            lfs: None,
            lfs_endpoint: None,
            milestones: None,
            mirror: None,
            mirror_interval: None,
            private: None,
            pull_requests: None,
            releases: None,
            repo_owner: None,
            service: None,
            wiki: None,
        }
    }

    /// Send the request to migrate a repository.
    pub async fn send(&self, client: &crate::Client) -> Result<Repository> {
        let req = client.post("repos/migrate").json(&self).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
