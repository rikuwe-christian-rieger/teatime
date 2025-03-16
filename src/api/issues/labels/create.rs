use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

/// Represents the options for creating a new user.
/// The only required field is `email` and `username`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct CreateLabelBuilder {
    #[build_it(skip)]
    #[skip]
    #[serde(skip)]
    pub owner: String,
    #[skip]
    #[serde(skip)]
    pub repo: String,
    #[build_it(skip)]
    /// Color of the label
    pub color: String,
    #[build_it(skip)]
    /// Name of the label
    pub name: String,
    /// Description of the label
    pub description: Option<String>,
    /// Whether the label is exclusive
    pub exclusive: Option<bool>,
    /// Whether the label is archived
    pub is_archived: Option<bool>,
}

impl CreateLabelBuilder {
    pub fn new(
        owner: impl ToString,
        repo: impl ToString,
        name: impl ToString,
        color: impl ToString,
    ) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            color: color.to_string(),
            name: name.to_string(),
            description: None,
            exclusive: None,
            is_archived: None,
        }
    }

    /// Send the request to create the user.
    /// This will return the created [User].
    pub async fn send(&self, client: &Client) -> Result<User> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .post(format!("repos/{owner}/{repo}/labels"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
