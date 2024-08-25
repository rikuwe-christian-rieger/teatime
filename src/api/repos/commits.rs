use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::repos::Commit};

/// Options for getting a list of commits from a repository.
/// All fields are optional.
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(default)]
pub struct GetCommitsBuilder {
    #[skip]
    #[serde(skip)]
    /// The owner of the repository to list commits for.
    owner: String,
    #[skip]
    #[serde(skip)]
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
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(sha) = &self.sha {
                params.append_pair("sha", sha);
            }
            if let Some(path) = &self.path {
                params.append_pair("path", path);
            }
            if let Some(stat) = &self.stat {
                params.append_pair("stat", &stat.to_string());
            }
            if let Some(verification) = &self.verification {
                params.append_pair("verification", &verification.to_string());
            }
            if let Some(files) = &self.files {
                params.append_pair("files", &files.to_string());
            }
            if let Some(page) = &self.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &self.limit {
                params.append_pair("limit", &limit.to_string());
            }
            if let Some(not) = &self.not {
                params.append_pair("not", not);
            }
        }
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
