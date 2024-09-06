use build_it::Builder;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::model::user::User;

/// Options for searching users.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct SearchUsersBuilder {
    /// Keyword to search for
    #[serde(rename = "q")]
    query: Option<String>,
    /// ID of the user to search for
    uid: Option<i64>,
    /// Page number of results to return (1-based)
    page: Option<i32>,
    /// Page size of results
    limit: Option<i32>,
}

impl SearchUsersBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Send the request to get the repository.
    /// This will return a [Vec<User>] object if the repository exists and is visible to the
    /// currently authenticated user.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<User>> {
        let req = client.get("users/search".to_string()).query(self).build()?;
        #[derive(Deserialize)]
        struct Response {
            #[allow(dead_code)]
            ok: bool,
            data: Vec<User>,
        }
        let res = client.make_request(req).await?;
        Ok(client.parse_response::<Response>(res).await?.data)
    }
}
