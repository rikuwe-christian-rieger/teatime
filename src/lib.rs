use core::fmt;
use std::{error::Error, fmt::Display};

use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeatimeErrorKind {
    AuthError,
    RepoCreateError,
}

impl Display for TeatimeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TeatimeErrorKind::AuthError => write!(f, "AuthError"),
            TeatimeErrorKind::RepoCreateError => write!(f, "RepoCreateError"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TeatimeError {
    pub message: String,
    pub status_code: reqwest::StatusCode,
}
impl Error for TeatimeError {}
impl Display for TeatimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

type Result<T> = std::result::Result<T, TeatimeError>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ObjectFormatName {
    #[serde(rename = "sha256")]
    #[default]
    SHA256,
    #[serde(rename = "sha1")]
    SHA1,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrustModel {
    #[serde(rename = "default")]
    #[default]
    Default,
    #[serde(rename = "collaborator")]
    Collaborator,
    #[serde(rename = "committer")]
    Committer,
    #[serde(rename = "collaboratorcommitter")]
    CollabroatorCommitter,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateRepoOption {
    pub auto_init: bool,
    pub default_branch: String,
    pub description: String,
    pub gitignores: String,
    pub issue_labels: String,
    pub license: String,
    pub name: String,
    pub object_format_name: ObjectFormatName,
    pub private: bool,
    pub readme: String,
    pub template: bool,
    pub trust_model: TrustModel,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GetCommitsOption {
    pub sha: Option<String>,
    pub path: Option<String>,
    #[serde(default = "default_true")]
    pub stat: bool,
    #[serde(default = "default_true")]
    pub verification: bool,
    #[serde(default = "default_true")]
    pub files: bool,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub not: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct User {
    pub active: bool,
    pub avatar_url: String,
    pub created: String,
    pub description: String,
    pub email: String,
    pub followers_count: i64,
    pub following_count: i64,
    pub full_name: String,
    pub id: i64,
    pub is_admin: bool,
    pub language: String,
    pub last_login: String,
    pub location: String,
    pub login: String,
    pub login_name: String,
    pub prohibit_login: bool,
    pub pronouns: String,
    pub restricted: bool,
    pub starred_repos_count: i64,
    pub visibility: String,
    pub website: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Repository {
    pub allow_fast_forward_only_merge: bool,
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_rebase_update: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    pub archived_at: String,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_allow_maintainer_edit: bool,
    pub default_branch: String,
    pub default_delete_branch_after_merge: bool,
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    pub fork: bool,
    pub forks_count: i64,
    pub full_name: String,
    pub has_actions: bool,
    pub has_issues: bool,
    pub has_packages: bool,
    pub has_projects: bool,
    pub has_pull_requests: bool,
    pub has_releases: bool,
    pub has_wiki: bool,
    pub html_url: String,
    pub id: i64,
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    pub language: String,
    pub languages_url: String,
    pub link: String,
    pub mirror: bool,
    pub mirror_interval: String,
    pub mirror_updated: String,
    pub name: String,
    pub object_format_name: ObjectFormatName,
    pub open_issues_count: i64,
    pub open_pr_counter: i64,
    pub original_url: String,
    pub owner: User,
    pub private: bool,
    pub release_counter: i64,
    pub size: i64,
    pub ssh_url: String,
    pub stars_count: i64,
    pub template: bool,
    pub updated_at: String,
    pub url: String,
    pub watchers_count: i64,
    pub website: String,
    pub wiki_branch: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommitUser {
    pub date: String,
    pub email: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RepoCommit {
    pub author: CommitUser,
    pub committer: CommitUser,
    pub message: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Commit {
    pub author: Option<User>,
    pub commit: RepoCommit,
    pub committer: Option<User>,
    pub html_url: String,
    pub sha: String,
    pub url: String,
}

pub struct Client {
    cli: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(base_url: String, token: String) -> Self {
        let mut headers = HeaderMap::new();
        let token = HeaderValue::from_str(&format!("token {}", token)).expect("token error");

        headers.insert(header::AUTHORIZATION, token);
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let cli = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .user_agent("teatime/0.0.1")
            .build()
            .expect("client build error");

        Self { cli, base_url }
    }

    pub async fn user_create_repository(
        &self,
        create_option: &CreateRepoOption,
    ) -> Result<Repository> {
        let req = self.post("user/repos").json(create_option).build()?;
        self.make_request(req).await
    }

    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository> {
        let req = self.get(format!("repos/{owner}/{repo}")).build()?;
        self.make_request(req).await
    }

    pub async fn delete_repository(&self, owner: &str, repo: &str) -> Result<()> {
        let req = self.delete(format!("repos/{owner}/{repo}")).build()?;
        self.make_request(req).await
    }

    pub async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        get_option: &GetCommitsOption,
    ) -> Result<Vec<Commit>> {
        let mut req = self.get(format!("repos/{owner}/{repo}/commits")).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(sha) = &get_option.sha {
                params.append_pair("sha", sha);
            }
            if let Some(path) = &get_option.path {
                params.append_pair("path", path);
            }
            params.append_pair("stat", &get_option.stat.to_string());
            params.append_pair("verification", &get_option.verification.to_string());
            params.append_pair("files", &get_option.files.to_string());
            if let Some(page) = &get_option.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &get_option.limit {
                params.append_pair("limit", &limit.to_string());
            }
            if let Some(not) = &get_option.not {
                params.append_pair("not", not);
            }
        }
        println!("{:#?}", req);
        let body = self.cli.execute(req).await?.text().await?;
        println!("{}", body);

        match serde_json::from_str(&body) {
            Ok(commits) => Ok(commits),
            Err(e) => Err(TeatimeError {
                message: format!("{}", e),
                status_code: StatusCode::BAD_REQUEST,
            }),
        }
    }

    pub fn delete(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::DELETE, path)
    }
    pub fn post(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::POST, path)
    }
    pub fn get(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::GET, path)
    }
    pub fn request_base(&self, method: Method, path: impl Display) -> reqwest::RequestBuilder {
        self.cli
            .request(method, format!("{}/api/v1/{}", self.base_url, path))
    }
    pub async fn make_request<T: DeserializeOwned>(&self, req: reqwest::Request) -> Result<T> {
        self.cli
            .execute(req)
            .await?
            .json::<T>()
            .await
            .map_err(|e| e.into())
    }
}

impl From<reqwest::Error> for TeatimeError {
    fn from(err: reqwest::Error) -> Self {
        TeatimeError {
            message: format!("{}", err),
            status_code: err.status().unwrap_or(StatusCode::BAD_REQUEST),
        }
    }
}

/// NOTE: This is a workaround for the janky `#[serde(default)]` attribute.
/// It's not possible to use `#[serde(default = true)]`, so we have to create this
/// helper function and use `#[serde(default = "default_true")]` instead.
/// This is a known issue: #368 (https://github.com/serde-rs/serde/issues/368)
pub const fn default_true() -> bool {
    true
}
