use crate::error::Result;
use crate::model::user::User;

#[derive(Default, Debug)]
pub struct GetAuthenticatedUserBuilder;

impl GetAuthenticatedUserBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Send the request to get the authenticated user.
    pub async fn send(&self, client: &crate::Client) -> Result<User> {
        // send the request
        let req = client.get("user").build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
