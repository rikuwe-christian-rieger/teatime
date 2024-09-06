use serde::Serialize;

use crate::{error::Result, model::pulls::PullRequest, Client};

#[derive(Debug, Clone, Serialize)]
pub struct PinnedPullRequestsBuilder {
    #[serde(skip)]
    owner: String,
    #[serde(skip)]
    repo: String,
}

impl PinnedPullRequestsBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
    /// Sends the request to list pinned pull requests.
    pub async fn send(&self, client: &Client) -> Result<Vec<PullRequest>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("/repos/{owner}/{repo}/pulls/pinned"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
