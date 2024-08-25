use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

/// Options for forking a repository.
/// All fields are optional.
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(default)]
pub struct CreateForkBuilder {
    /// The owner of the repository to fork.
    /// This is the user or organization that owns the repository you want to fork.
    #[skip]
    #[serde(skip)]
    owner: String,
    /// The name of the repository to fork.
    #[skip]
    #[serde(skip)]
    repo: String,
    /// The name of the new repository.
    /// Will be the same as the original if not set.
    name: Option<String>,
    /// Organization name, if forking into an organization.
    organization: Option<String>,
}

impl CreateForkBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            organization: None,
            name: None,
        }
    }
    /// Send the request to fork the repository.
    pub async fn send(&self, client: &Client) -> Result<User> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .post(format!("repos/{owner}/{repo}/forks"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

/// Options for listing a repository's forks.
/// All fields are optional.
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(default)]
pub struct ListForksBuilder {
    #[skip]
    #[serde(skip)]
    /// The owner of the repository to list forks for.
    owner: String,
    #[skip]
    #[serde(skip)]
    /// The name of the repository to list forks for.
    repo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional page number of the results to fetch (1-based).
    /// Defaults to 1 if not set.
    page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional number of forks to return per page (page-size).
    /// Defaults to the maximum your instance allows if not set.
    limit: Option<i64>,
}

impl ListForksBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            page: None,
            limit: None,
        }
    }
    /// Send the request to list the forks.
    pub async fn send(&self, client: &Client) -> Result<Vec<User>> {
        let ListForksBuilder {
            page,
            limit,
            owner,
            repo,
        } = self;

        let mut req = client.get(format!("repos/{owner}/{repo}/forks")).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(page) = page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
