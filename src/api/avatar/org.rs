use build_it::Builder;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{error::Result, Client};

#[derive(Debug, Serialize, Clone, Deserialize, Builder)]
#[build_it(into)]
pub struct UpdateOrgAvatarBuilder {
    #[skip]
    name: String,
    #[skip]
    image: String,
}

impl UpdateOrgAvatarBuilder {
    pub fn new(name: impl ToString, image: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            image: image.to_string(),
        }
    }

    pub async fn send(&self, client: &Client) -> Result<StatusCode> {
        let req = client
            .post(format!("orgs/{}/avatar", self.name))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        Ok(res.status())
    }
}
