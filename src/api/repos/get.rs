use crate::{error::Result, model::repos::Repository};

#[derive(Debug)]
pub struct GetRepoBuilder {
    owner: String,
    repo: String,
}

impl GetRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }
    /// Send the request to get the repository.
    /// This will return a [Repository] object if the repository exists and is visible to the
    /// currently authenticated user.
    pub async fn send(&self, client: &crate::Client) -> Result<Repository> {
        let GetRepoBuilder { owner, repo } = self;
        let req = client.get(format!("repos/{owner}/{repo}")).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
