use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Branch, Client};

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct ListBranchesBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,

    /// Page number of results to return (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i64>,

    /// Page size of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Builder)]
#[build_it(into)]
pub struct CreateBranchBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    owner: String,
    #[serde(skip)]
    #[build_it(skip)]
    repo: String,
    #[build_it(skip)]
    new_branch_name: String,

    old_ref_name: Option<String>,
}

#[derive(Debug, Clone, Builder)]
#[build_it(into)]
pub struct GetBranchBuilder {
    #[build_it(skip)]
    owner: String,
    #[build_it(skip)]
    repo: String,
    #[build_it(skip)]
    branch: String,
}

#[derive(Debug, Clone, Builder)]
#[build_it(into)]
pub struct DeleteBranchBuilder {
    #[build_it(skip)]
    owner: String,
    #[build_it(skip)]
    repo: String,
    #[build_it(skip)]
    branch: String,
}

impl ListBranchesBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list a repository's branches.
    pub async fn send(&self, client: &Client) -> Result<Vec<Branch>> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .get(format!("repos/{owner}/{repo}/branches"))
            .query(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl CreateBranchBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, new_branch_name: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            new_branch_name: new_branch_name.to_string(),
            old_ref_name: None,
        }
    }
    /// Sends the request to create a branch.
    pub async fn send(&self, client: &Client) -> Result<Branch> {
        let owner = &self.owner;
        let repo = &self.repo;
        let req = client
            .post(format!("repos/{owner}/{repo}/branches"))
            .json(&self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl GetBranchBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, branch: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            branch: branch.to_string(),
        }
    }
    /// Sends the request to get a branch.
    pub async fn send(&self, client: &Client) -> Result<Branch> {
        let Self {
            owner,
            repo,
            branch,
        } = self;
        let req = client
            .get(format!("repos/{owner}/{repo}/branches/{branch}"))
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl DeleteBranchBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString, branch: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            branch: branch.to_string(),
        }
    }
    /// Sends the request to get a branch.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self {
            owner,
            repo,
            branch,
        } = self;
        let req = client
            .delete(format!("repos/{owner}/{repo}/branches/{branch}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}
