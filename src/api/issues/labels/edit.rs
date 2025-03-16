use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::issues::Label, Client};

/// Represents the options for creating a new user.
/// The only required field is `email` and `username`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct EditRepoLabelBuilder {
    #[build_it(skip)]
    #[skip]
    #[serde(skip)]
    pub owner: String,
    #[skip]
    #[serde(skip)]
    pub repo: String,
    #[skip]
    #[serde(skip)]
    pub id: i64,

    /// Color of the label
    pub color: Option<String>,
    /// Name of the label
    pub name: Option<String>,
    /// Description of the label
    pub description: Option<String>,
    /// Whether the label is exclusive
    pub exclusive: Option<bool>,
    /// Whether the label is archived
    pub is_archived: Option<bool>,
}

impl EditRepoLabelBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, id: i64) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            id,
            color: None,
            name: None,
            description: None,
            exclusive: None,
            is_archived: None,
        }
    }

    /// Send the request to create the label.
    /// This will return the created [Label].
    pub async fn send(&self, client: &Client) -> Result<Label> {
        let owner = &self.owner;
        let repo = &self.repo;
        let id = &self.id;
        let req = client
            .patch(format!("repos/{owner}/{repo}/labels/{id}"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
