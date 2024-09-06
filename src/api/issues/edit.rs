use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Issue};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct EditIssueBuilder {
    #[skip]
    #[serde(skip)]
    pub owner: String,
    #[skip]
    #[serde(skip)]
    pub repo: String,
    #[skip]
    #[serde(skip)]
    pub issue_number: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[build_it(rename = "refs")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unset_due_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: Make this a DateTime<Utc>
    pub updated_at: Option<String>,
}

impl EditIssueBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, issue_number: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            issue_number,
            assignees: None,
            body: None,
            due_date: None,
            milestone: None,
            r#ref: None,
            title: None,
            unset_due_date: None,
            updated_at: None,
        }
    }

    /// Send the request to edit the issue.
    pub async fn send(&self, client: &crate::Client) -> Result<Issue> {
        // send the request
        let owner = &self.owner;
        let repo = &self.repo;
        let index = &self.issue_number;
        let req = client
            .patch(format!("repos/{owner}/{repo}/issues/{index}"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
