use crate::error::Result;
use crate::model::user::User;

pub struct GetUserBuilder {
    username: String,
}

impl GetUserBuilder {
    pub fn new(username: impl ToString) -> Self {
        Self {
            username: username.to_string(),
        }
    }
    pub async fn send(&self, client: &crate::Client) -> Result<User> {
        let req = client.get(format!("users/{}", self.username)).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
