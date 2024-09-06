use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::pulls::PullRequest, Client};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct CreatePullRequestBuilder {
    #[serde(skip)]
    #[skip]
    owner: String,
    #[serde(skip)]
    #[skip]
    repo: String,

    #[skip]
    base: String,
    #[skip]
    head: String,
    #[skip]
    title: String,

    assignees: Option<Vec<String>>,
    body: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<i64>>,
    milestone: Option<i64>,
}

impl CreatePullRequestBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        head: impl ToString,
        base: impl ToString,
        title: impl ToString,
    ) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            base: base.to_string(),
            head: head.to_string(),
            title: title.to_string(),
            assignees: None,
            body: None,
            due_date: None,
            labels: None,
            milestone: None,
        }
    }

    /// Sends the request to create a pull request
    pub async fn send(&self, client: &Client) -> Result<PullRequest> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .post(format!("repos/{owner}/{repo}/pulls",))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
