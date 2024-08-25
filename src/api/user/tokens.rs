use serde::Serialize;

use crate::error::Result;
use crate::model::user::AccessToken;

#[derive(Debug, Clone, Serialize)]
pub struct CreateAccessTokenBuilder {
    #[serde(skip)]
    /// The username of the user to create the access token for.
    pub user: String,
    /// Access token name.
    pub name: String,
    /// Optional scopes for the access token.
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteAccessTokenBuilder {
    /// The username of the user to delete the access token for.
    pub user: String,
    /// The access token to delete.
    pub token: String,
}

impl CreateAccessTokenBuilder {
    pub fn new(user: impl ToString, name: impl ToString, scopes: Vec<impl ToString>) -> Self {
        Self {
            user: user.to_string(),
            name: name.to_string(),
            scopes: scopes.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Sends the request to create the access token.
    pub async fn send(&self, client: &crate::Client) -> Result<AccessToken> {
        let username = &self.user;
        let req = client
            .post(format!("users/{username}/tokens"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl DeleteAccessTokenBuilder {
    pub fn new(user: impl ToString, token: impl ToString) -> Self {
        Self {
            user: user.to_string(),
            token: token.to_string(),
        }
    }
    /// Sends the request to delete the access token.
    pub async fn send(&self, client: &crate::Client) -> Result<()> {
        let DeleteAccessTokenBuilder { user, token } = self;
        let req = client
            .delete(format!("users/{user}/tokens/{token}"))
            .build()?;
        client.make_request(req).await?;
        Ok(())
    }
}
