use build_it::Builder;
use serde::Serialize;

use crate::{
    error::Result,
    model::{issues::StateType, pulls::PullRequest},
    Client,
};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct EditPullRequestBuilder {
    #[serde(skip)]
    #[skip]
    owner: String,
    #[serde(skip)]
    #[skip]
    repo: String,
    #[serde(skip)]
    #[skip]
    id: i64,

    allow_maintainer_edit: Option<bool>,
    assignees: Option<Vec<String>>,
    base: Option<String>,
    body: Option<String>,
    due_date: Option<String>,
    labels: Option<Vec<i64>>,
    milestone: Option<i64>,
    state: Option<StateType>,
    title: Option<String>,
    unset_due_date: Option<bool>,
}

impl EditPullRequestBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, id: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            id,
            allow_maintainer_edit: None,
            assignees: None,
            base: None,
            body: None,
            due_date: None,
            labels: None,
            milestone: None,
            state: None,
            title: None,
            unset_due_date: None,
        }
    }

    /// Sends the request to edit a pull request
    pub async fn send(&self, client: &Client) -> Result<PullRequest> {
        let owner = &self.owner;
        let repo = &self.repo;
        let id = self.id;
        let req = client
            .patch(format!("repos/{owner}/{repo}/pulls/{id}"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
