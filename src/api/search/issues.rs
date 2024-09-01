use build_it::Builder;
use serde::Serialize;
use teatime_macros::QueryParams;

use crate::error::Result;
use crate::model::issues::{Issue, IssueType, State};

/// Options for searching issues.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Builder, QueryParams)]
pub struct SearchIssuesBuilder {
    /// Filter by open or closed issues
    state: Option<State>,
    /// Filter issues by labels. Non-existent labels are ignored.
    #[query_params(skip)]
    labels: Option<Vec<String>>,
    /// Filter issues by milestone names. Non-existent milestones are ignored.
    #[query_params(skip)]
    milestones: Option<Vec<String>>,
    /// Search string
    #[query_params(rename = "q")]
    query: Option<String>,
    /// Repository to prioritize in the results
    priority_repo_id: Option<i64>,
    /// Filter by type (issue or pull request) if set
    #[query_params(rename = "type")]
    issue_type: Option<IssueType>,
    /// Only show issues updated after the given time. This is a timestamp in RFC 3339 format.
    // TODO: Make this a DateTime<Utc>
    since: Option<String>,
    /// Only show issues updated before the given time. This is a timestamp in RFC 3339 format.
    // TODO: Make this a DateTime<Utc>
    before: Option<String>,
    /// Filter issues/PRs assigned to the authenticated user, default is false
    assigned: Option<bool>,
    /// Filter issues/PRs created by the authenticated user, default is false
    created: Option<bool>,
    /// Filter issues/PRs in which the authenticated user is mentioned, default is false
    mentioned: Option<bool>,
    /// Filter pull requests awaiting review by the authenticated user, default is false
    review_requested: Option<bool>,
    /// Filter pull requests reviewed by the authenticated user, default is false
    reviewed: Option<bool>,
    /// Filter by owner
    owner: Option<String>,
    /// Filter by team
    team: Option<String>,
    /// Page number of results to return (1-based)
    page: Option<i32>,
    /// Page size of results
    limit: Option<i32>,
}

impl SearchIssuesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Send the request to search for issues.
    /// This will return a [Vec<Issue>] of all issues matching the search criteria.
    /// Only shows issues the currently authenticated user can see.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Issue>> {
        let mut req = client.get("repos/issues/search".to_string()).build()?;
        self.append_query_params(&mut req);
        if let Some(labels) = &self.labels {
            req.url_mut()
                .query_pairs_mut()
                .append_pair("labels", &labels.join(","));
        }
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
