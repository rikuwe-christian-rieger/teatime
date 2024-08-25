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
        let mut req = client.get(format!("repos/{owner}/{repo}/issues")).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();
            if let Some(state) = &self.state {
                params.append_pair("state", &state.to_string());
            }
            if let Some(labels) = &self.labels {
                params.append_pair("labels", &labels.join(","));
            }
            if let Some(query) = &self.query {
                params.append_pair("q", query);
            }
            if let Some(issue_type) = &self.issue_type {
                params.append_pair("type", &issue_type.to_string());
            }
            if let Some(milestone) = &self.milestone {
                params.append_pair("milestone", milestone);
            }
            if let Some(since) = &self.since {
                params.append_pair("since", since);
            }
            if let Some(before) = &self.before {
                params.append_pair("before", before);
            }
            if let Some(created_by) = &self.created_by {
                params.append_pair("created_by", created_by);
            }
            if let Some(assigned_by) = &self.assigned_by {
                params.append_pair("assigned_by", assigned_by);
            }
            if let Some(mentioned_by) = &self.mentioned_by {
                params.append_pair("mentioned_by", mentioned_by);
            }
            if let Some(page) = &self.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &self.limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
