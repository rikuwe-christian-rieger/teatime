use crate::error::Result;

pub struct DeleteIssueBuilder {
    owner: String,
    repo: String,
    issue_number: i64,
}

impl DeleteIssueBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, issue_number: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            issue_number,
        }
    }
    pub async fn send(&self, client: &crate::Client) -> Result<()> {
        let DeleteIssueBuilder {
            owner,
            repo,
            issue_number,
        } = self;
        let req = client
            .delete(format!("repos/{owner}/{repo}/issues/{issue_number}",))
            .build()?;
        client.make_request(req).await?;
        Ok(())
    }
}
