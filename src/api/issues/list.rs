use build_it::Builder;
use serde::Serialize;

use crate::error::Result;
use crate::model::issues::{Issue, IssueType, State};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct ListIssuesBuilder {
    #[skip]
    #[serde(skip)]
    owner: String,
    #[skip]
    #[serde(skip)]
    repo: String,

    /// Whether issue is open or closed
    pub state: Option<State>,
    /// Comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
    pub labels: Option<Vec<String>>,
    /// Search string
    #[serde(rename = "q")]
    pub query: Option<String>,
    /// Filter by type (Issues or Pulls) if set
    #[serde(rename = "type")]
    pub issue_type: Option<IssueType>,
    /// Comma-separated list of milestone names or ids. It uses names and fall back to ids.
    /// Fetch only issues that have any of this milestones. Non existent milestones are discarded
    pub milestone: Option<String>,
    /// Only show items updated after the given time. This is a timestamp in RFC 3339 format
    pub since: Option<String>,
    /// Only show items updated before the given time. This is a timestamp in RFC 3339 format
    pub before: Option<String>,
    /// Only show items which were created by the given user
    pub created_by: Option<String>,
    /// Only show items for which the given user is assigned
    pub assigned_by: Option<String>,
    /// Only show items in which the given user was mentioned
    pub mentioned_by: Option<String>,
    /// Page number of results to return (1-based)
    pub page: Option<i64>,
    /// Page size of results
    pub limit: Option<i64>,
}

impl ListIssuesBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            state: None,
            labels: None,
            query: None,
            issue_type: None,
            milestone: None,
            since: None,
            before: None,
            created_by: None,
            assigned_by: None,
            mentioned_by: None,
            page: None,
            limit: None,
        }
    }
    /// Send the request to get the issues.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Issue>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("repos/{owner}/{repo}/issues"))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
