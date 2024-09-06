use crate::{error::Result, model::pulls::PullRequest, Client};

#[derive(Debug, Clone)]
pub struct GetPullRequestByIdBuilder {
    owner: String,
    repo: String,
    id: i64,
}

impl GetPullRequestByIdBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, id: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            id,
        }
    }
    /// Sends the request to get a pull request by its ID.
    pub async fn send(&self, client: &Client) -> Result<PullRequest> {
        let Self { owner, repo, id } = self;
        let req = client
            .get(format!("/repos/{owner}/{repo}/pulls/{id}"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

#[derive(Debug, Clone)]
pub struct GetPullRequestByBranchesBuilder {
    owner: String,
    repo: String,
    head: String,
    base: String,
}

impl GetPullRequestByBranchesBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        head: impl ToString,
        base: impl ToString,
    ) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            head: head.to_string(),
            base: base.to_string(),
        }
    }
    /// Sends the request to get a pull request by its head and base branches.
    pub async fn send(&self, client: &Client) -> Result<PullRequest> {
        let Self {
            owner,
            repo,
            head,
            base,
        } = self;
        let req = client
            .get(format!("/repos/{owner}/{repo}/pulls/{base}/{head}"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
