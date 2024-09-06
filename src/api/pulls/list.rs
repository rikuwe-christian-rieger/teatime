use build_it::Builder;
use serde::Serialize;

use crate::{
    error::Result,
    model::{
        issues::State,
        pulls::{PullRequest, Sort},
    },
    Client,
};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct ListPullRequestsBuilder {
    #[skip]
    #[serde(skip)]
    owner: String,
    #[skip]
    #[serde(skip)]
    repo: String,

    state: Option<State>,
    sort: Option<Sort>,
    milestone: Option<i64>,
    /// Label IDs
    labels: Option<Vec<i64>>,
    /// Page number of results to return (1-based)
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

impl ListPullRequestsBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            state: None,
            sort: None,
            milestone: None,
            labels: None,
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list pull requests.
    pub async fn send(&self, client: &Client) -> Result<Vec<PullRequest>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("/repos/{owner}/{repo}/pulls"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
