use crate::error::Result;
use crate::model::issues::Issue;

#[derive(Debug, Clone)]
pub struct GetIssueBuilder {
    owner: String,
    repo: String,
    issue_number: i64,
}

impl GetIssueBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, issue_number: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            issue_number,
        }
    }
    /// Send the request to get the issues.
    pub async fn send(&self, client: &crate::Client) -> Result<Issue> {
        let owner = &self.owner;
        let repo = &self.repo;
        let index = &self.issue_number;
        let req = client
            .get(format!("repos/{owner}/{repo}/issues/{index}"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
