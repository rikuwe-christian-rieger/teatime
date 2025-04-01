use crate::{error::Result, Client};
use build_it::Builder;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Builder)]
#[build_it(into)]
pub struct UpdateRepoAvatarBuilder {
    #[skip]
    owner: String,
    #[skip]
    repo: String,
    #[skip]
    image: String,
}

impl UpdateRepoAvatarBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, image: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            image: image.to_string(),
        }
    }

    pub async fn send(&self, client: &Client) -> Result<StatusCode> {
        let req = client
            .post(format!("repos/{}/{}/avatar", self.owner, self.repo))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        Ok(res.status())
    }
}
