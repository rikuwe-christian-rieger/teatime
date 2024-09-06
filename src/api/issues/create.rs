use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Issue};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct CreateIssueBuilder {
    #[skip]
    #[serde(skip)]
    pub owner: String,
    #[skip]
    #[serde(skip)]
    pub repo: String,
    #[skip]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[build_it(rename = "refs")]
    pub r#ref: Option<String>,
}

impl CreateIssueBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, title: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            title: title.to_string(),
            assignees: None,
            body: None,
            closed: None,
            due_date: None,
            labels: None,
            milestone: None,
            r#ref: None,
        }
    }

    /// Send the request to create the issue.
    pub async fn send(&self, client: &crate::Client) -> Result<Issue> {
        // send the request
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .post(format!("repos/{owner}/{repo}/issues"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
