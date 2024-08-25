use crate::Result;

#[derive(Debug)]
pub struct DeleteRepoBuilder {
    owner: String,
    repo: String,
}

impl DeleteRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Send the request to delete the repository.
    pub async fn send(&self, client: &crate::Client) -> Result<()> {
        let DeleteRepoBuilder { owner, repo } = self;
        let req = client.delete(format!("repos/{owner}/{repo}")).build()?;
        client.make_request(req).await?;
        Ok(())
    }
}
