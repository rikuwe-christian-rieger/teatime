use build_it::Builder;
use serde::Serialize;
use teatime_macros::QueryParams;

use crate::{error::Result, model::repos::Commit};

/// Options for getting a list of commits from a repository.
/// All fields are optional.
#[derive(Debug, Clone, Serialize, Builder, QueryParams)]
#[serde(default)]
pub struct GetCommitsBuilder {
    #[skip]
    #[serde(skip)]
    #[query_params(skip)]
    /// The owner of the repository to list commits for.
    owner: String,
    #[skip]
    #[serde(skip)]
    #[query_params(skip)]
    /// The name of the repository to list commits for.
    repo: String,

    /// SHA or branch to start listing commits from (usually the default branch).
    pub sha: Option<String>,
    /// File path to a file/directory in the repository.
    /// If provided, only commits affecting this path will be returned.
    pub path: Option<String>,
    /// Whether to include the `stat` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    pub stat: Option<bool>,
    /// Whether to include the `verification` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    pub verification: Option<bool>,
    /// Whether to include the `files` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    pub files: Option<bool>,
    /// Optional page number of the results to fetch (1-based).
    /// Defaults to 1 if not set.
    pub page: Option<i64>,
    /// Optional number of commits to return per page (page-size).
    /// Defaults to the maximum your instance allows if not set.
    pub limit: Option<i64>,
    /// Commits that match the given specifier will not be listed.
    pub not: Option<String>,
}

impl GetCommitsBuilder {
    pub fn new(owner: impl ToString, repo: impl ToString) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            sha: None,
            path: None,
            stat: None,
            verification: None,
            files: None,
            page: None,
            limit: None,
            not: None,
        }
    }

    /// Send the request to get the commits.
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Commit>> {
        let owner = &self.owner;
        let repo = &self.repo;

        let mut req = client
            .get(format!("repos/{owner}/{repo}/commits"))
            .build()?;
        self.append_query_params(&mut req);
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
