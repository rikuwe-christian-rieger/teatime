use build_it::Builder;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{error::Result, model::repos::Repository, Client};

#[derive(Default, Debug, Clone, Serialize, Builder)]
pub struct ListStarredBuilder {
    /// Page number of the results to return (1-based).
    page: Option<i64>,
    /// Page size of results
    limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct StarRepoBuilder {
    owner: String,
    repo: String,
}

#[derive(Debug, Clone)]
pub struct UnstarRepoBuilder {
    owner: String,
    repo: String,
}

#[derive(Debug, Clone)]
pub struct IsStarredBuilder {
    owner: String,
    repo: String,
}

impl ListStarredBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sends the request to list the user's starred repos.
    pub async fn send(&self, client: &Client) -> Result<Vec<Repository>> {
        let req = client.get("/user/starred").query(self).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl StarRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Sends the request to star the repo.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self { owner, repo } = self;
        let req = client
            .put(format!("/user/starred/{owner}/{repo}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}

impl UnstarRepoBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Sends the request to star the repo.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self { owner, repo } = self;
        let req = client
            .delete(format!("/user/starred/{owner}/{repo}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}

impl IsStarredBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    /// Sends the request to star the repo.
    pub async fn send(&self, client: &Client) -> Result<bool> {
        let Self { owner, repo } = self;
        let req = client
            .get(format!("/user/starred/{owner}/{repo}"))
            .build()?;
        match client.make_request(req).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.status_code == StatusCode::NOT_FOUND {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }
}
