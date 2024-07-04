//! This crate is a simple Gitea API client. It's goal is to give you the ability to write
//! exactly as much code as you need to interact with the specific parts of the Gitea API you need,
//! but no more.
//!
//! # Usage
//! The main way to interact with the Gitea API is through the `Client` struct. You can create a
//! new [Client] by calling [Client::new] with the base URL of your Gitea instance and a personal
//! token. Teatime does currently not support basic HTML or OAuth2 authentication.
//!
//! Once you have obtained a [Client], you can interact with the Gitea API by calling the various
//! methods the instance provides. For example, to create a new repository, you can call:
//! ```rust
//! # use teatime::{Client, CreateRepoOption};
//! # async fn create_repo() {
//! let client = Client::new("https://gitea.example.com".to_string(), "your-token".to_string());
//! let create_option = CreateRepoOption {
//!     // `name` is the only required field
//!     name: "my-new-repo".to_string(),
//!     ..Default::default()
//! };
//! // This will create a new repository with the name "my-new-repo" for the authenticated user.
//! let repo = client.user_create_repository(&create_option).await.unwrap();
//! # }
//! ```
//!
//! Similarly, to get a list of commits for a repository, you can call:
//! ```rust
//! # use teatime::{Client, GetCommitsOption};
//! # async fn get_commits() {
//! let client = Client::new("https://gitea.example.com".to_string(), "your-token".to_string());
//! let get_option = GetCommitsOption {
//!     // `GetCommitsOption` has a number of optional fields to filter the results,
//!     // but none are required. In this example, we're just setting the `limit` to 10 to
//!     // only get the 10 most recent commits.
//!     limit: Some(10),
//!     ..Default::default()
//! };
//! let commits = client.get_commits("owner", "repo", &get_option).await.unwrap();
//! # }
//! ```
//!
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

/// Represents some kind of error that can occur when interacting with the Gitea API.
/// This simply wraps a message and a status code.
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

/// A type alias for a [std::result::Result] that uses [TeatimeError] as the error type.
/// We define this purely for convenience.
type Result<T> = std::result::Result<T, TeatimeError>;

/// Represents the format of the object in the repository.
/// Defaults to [ObjectFormatName::SHA1].
/// SHA1 is more widely supported, but SHA256 is more secure.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ObjectFormatName {
    #[default]
    #[serde(rename = "sha1")]
    SHA1,
    #[serde(rename = "sha256")]
    SHA256,
}

/// Represents the trust model for verifying commits in the repository.
/// Defaults to [TrustModel::Default] (obviously).
/// This determines when signatures are considered "trusted".
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrustModel {
    /// Use the default repository trust model for this installation.
    #[serde(rename = "default")]
    #[default]
    Default,
    /// Trust signatures signed by keys of collaborators.
    #[serde(rename = "collaborator")]
    Collaborator,
    /// Trust signatures that match the commiters (This matches GitHub and will force Gitea signed
    /// commits to have Gitea as the committer).
    #[serde(rename = "committer")]
    Committer,
    /// Trust signatures signed by keys of collaborators which match the committer.
    #[serde(rename = "collaboratorcommitter")]
    CollabroatorCommitter,
}

/// Represents the options for creating a new repository.
/// The only required field is `name`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateRepoOption {
    /// Whether the repository should be automatically initialized.
    /// This will create a README, LICENSE, and .gitignore file.
    pub auto_init: bool,
    /// Default branch of the repository.
    pub default_branch: String,
    /// Description of the repository.
    pub description: String,
    /// Optional Gitignore templates to use.
    /// Will be ignored if `auto_init` is false.
    pub gitignores: String,
    /// Optional Issue label-set to use.
    pub issue_labels: String,
    /// Optional LICENSE to use.
    /// Will be ignored if `auto_init` is false.
    pub license: String,
    /// Name of the repository to create.
    /// NOTE: This field is required. Not setting it will result in an error upon
    /// repository creation.
    pub name: String,
    /// Object Format Name of the underlying git repository.
    pub object_format_name: ObjectFormatName,
    /// Whether the repository is private.
    /// Defaults to false.
    pub private: bool,
    /// Optional README template to use.
    /// Will be ignored if `auto_init` is false.
    pub readme: String,
    /// Whether the repository is a template.
    /// Defaults to false.
    pub template: bool,
    /// Trust model for verifying commits in the repository.
    /// Defaults to [TrustModel::Default].
    pub trust_model: TrustModel,
}

/// Options for getting a list of commits from a repository.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GetCommitsOption {
    /// SHA or branch to start listing commits from (usually the default branch).
    pub sha: Option<String>,
    /// File path to a file/directory in the repository.
    /// If provided, only commits affecting this path will be returned.
    pub path: Option<String>,
    /// Whether to include the `stat` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    #[serde(default = "default_true")]
    pub stat: bool,
    /// Whether to include the `verification` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    #[serde(default = "default_true")]
    pub verification: bool,
    /// Whether to include the `files` field in the response.
    /// Disable to speed-up the response.
    /// Defaults to true.
    /// NOTE: Commit verification is not implemented yet, so this setting does nothing.
    #[serde(default = "default_true")]
    pub files: bool,
    /// Optional page number of the results to fetch (1-based).
    /// Defaults to 1 if not set.
    pub page: Option<i64>,
    /// Optional number of commits to return per page (page-size).
    /// Defaults to the maximum your instance allows if not set.
    pub limit: Option<i64>,
    /// Commits that match the given specifier will not be listed.
    pub not: Option<String>,
}

/// Represents a Gitea user.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct User {
    /// Whether the user is active.
    pub active: bool,
    /// URL to the user's avatar.
    pub avatar_url: String,
    /// Date the user was created at.
    pub created: String,
    /// Description of the user (empty string if the user did not provide a discription).
    pub description: String,
    /// Email of the user.
    pub email: String,
    /// Number of followers the user has.
    pub followers_count: i64,
    /// Number of users the user is following.
    pub following_count: i64,
    /// Full name of the user.
    pub full_name: String,
    /// ID of the user.
    pub id: i64,
    /// Whether the user is an admin.
    pub is_admin: bool,
    /// Language the user speaks (empty string if the user did not specify any languaged).
    pub language: String,
    /// Date the user last logged in.
    pub last_login: String,
    /// Location of the user (empty string if the user did not provide a location).
    pub location: String,
    /// The user's username
    pub login: String,
    /// The user's authenticated sign-in name. Empty by default.
    pub login_name: String,
    /// Whether the user is prohibited from logging in.
    pub prohibit_login: bool,
    /// Pronouns of the user (empty string if the user did not provide any pronouns).
    pub pronouns: String,
    /// Whether the user is restricted.
    pub restricted: bool,
    /// Number of repositories the user has starred.
    pub starred_repos_count: i64,
    /// User visibility.
    /// Can be one of "public", "limited", or "private".
    pub visibility: String,
    /// The user's website (empty string if the user did not provide a website).
    pub website: String,
}

/// Represents a Gitea repository.
/// This struct is a subset of the full repository object.
/// Some fields the API provides (like external trackers) are not included here.
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

/// Represents information about a user in the context of a commit.
/// NOTE: This is not the same as the [User] struct.
/// A CommitUser is not guaranteed to be a valid Gitea user.
/// A commit author can set the name and email tracked in this struct to anything they want.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommitUser {
    /// Date the commit was authored.
    pub date: String,
    /// Email of the user.
    pub email: String,
    /// Full name of the user.
    pub name: String,
}

/// Represents the actual commit object in the underlying git repository.
/// This struct is a subset of the full commit object.
/// It does not include the full commit tree or commit verification.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RepoCommit {
    /// Author of the commit (usually the person who originally wrote the code).
    pub author: CommitUser,
    /// The person who committed the code on behalf of the author. May be the same as the author.
    pub committer: CommitUser,
    /// The commit message.
    pub message: String,
    /// The API endpoint for the commit
    /// (https://gitea-host.com/api/v1/repos/{user}/{repo}/git/commits/{sha}.
    pub url: String,
}

/// Represents a commit in a repository.
/// This struct is a subset of the full commit object.
/// It does not include the affected files, parent commits or commit stats (additions and
/// deletions).
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Commit {
    /// The commit author's Gitea account.
    /// See [RepoCommit::author] for more information.
    /// NOTE: This is not guaranteed to be a valid Gitea user.
    /// Because of the nature of Git, this field can be null
    pub author: Option<User>,
    pub commit: RepoCommit,
    /// The committer's Gitea account.
    /// See [RepoCommit::committer] for more information.
    /// NOTE: This is not guaranteed to be a valid Gitea user.
    /// Because of the nature of Git, this field can be null
    pub committer: Option<User>,
    /// The URL to the commit on the Gitea instance.
    pub html_url: String,
    /// The SHA of the commit.
    pub sha: String,
    /// The API endpoint URL for the commit.
    pub url: String,
}

/// Represents a Gitea client.
/// This struct is the main way to interact with the Gitea API.
/// It provides methods for creating repositories, getting repositories, deleting repositories,
/// and listing a repo's commits.
pub struct Client {
    cli: reqwest::Client,
    base_url: String,
}

impl Client {
    /// Creates a new Gitea client with the given base URL and personal token.
    /// NOTE: The base URL MUST not include the `/api/v1` path and should not contain any trailing
    /// slashes. For example, `https://gitea.example.com` is a valid base URL, but
    /// `https://gitea.example.com/` or `https://gitea.example.com/api/v1` are not.
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

    /// Creates a new repository for the authenticated user.
    /// The only required field in the [CreateRepoOption] is `name`.
    /// All other fields are optional.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, CreateRepoOption};
    /// # async fn create_repo() {
    /// let client = Client::new("https://gitea.example.com".to_string(),
    /// "5fda63fdbbfcfd131607881cda5e1d28a215e9e1".to_string());
    /// let create_option = CreateRepoOption {
    ///    // `name` is the only required field
    ///    name: "my-new-repo".to_string(),
    ///    ..Default::default()
    /// };
    /// // This will create a new repository with the name "my-new-repo" for the authenticated
    /// // user.
    /// let repo = client.user_create_repository(&create_option).await.unwrap();
    /// # }
    pub async fn user_create_repository(
        &self,
        create_option: &CreateRepoOption,
    ) -> Result<Repository> {
        let req = self.post("user/repos").json(create_option).build()?;
        self.make_request(req).await
    }

    /// Gets a repository by its owner and name.
    /// This will return a [Repository] object if the repository exists and is visible to the
    /// currently authenticated user.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::Client;
    /// # async fn get_repo() {
    /// let client = Client::new("https://gitea.example.com".to_string(),
    /// "793eae2c1dcd71daf9e6cc0f8a448a39b45d3ff3".to_string());
    /// let repo = client.get_repository("owner", "repo").await.unwrap();
    /// # }
    /// ```
    ///
    pub async fn get_repository(&self, owner: &str, repo: &str) -> Result<Repository> {
        let req = self.get(format!("repos/{owner}/{repo}")).build()?;
        self.make_request(req).await
    }

    /// Deletes a repository by its owner and name.
    /// WARNING: This is irreversible and will delete all data associated with the repository.
    /// This action cannot be undone. When invoking this method, you will not be asked for
    /// confirmation. Use with caution, please don't nuke your repositories.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::Client;
    /// # async fn delete_repo() {
    /// let client = Client::new("https://gitea.example.com".to_string(),
    /// "e8ffd828994fc890156c56004e9f16eef224d8b0".to_string());
    /// client.delete_repository("owner", "repo").await.unwrap();
    /// # }
    pub async fn delete_repository(&self, owner: &str, repo: &str) -> Result<()> {
        let req = self.delete(format!("repos/{owner}/{repo}")).build()?;
        self.make_request(req).await
    }

    /// Gets a list of commits for a repository.
    /// The [GetCommitsOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent commits for the default branch.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, GetCommitsOption};
    /// # async fn get_commits() {
    /// let client = Client::new("https://gitea.example.com".to_string(),
    /// "e8ffd828994fc890156c56004e9f16eef224d8b0".to_string());
    /// let get_option = GetCommitsOption::default();
    /// let commits = client.get_commits("owner", "repo", &get_option).await.unwrap();
    /// # }
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

    /// Creates a new DELETE-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn delete(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::DELETE, path)
    }
    /// Creates a new POST-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn post(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::POST, path)
    }
    /// Creates a new POST-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn get(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::GET, path)
    }
    /// Creates a new request builder with the given method and path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn request_base(&self, method: Method, path: impl Display) -> reqwest::RequestBuilder {
        self.cli
            .request(method, format!("{}/api/v1/{}", self.base_url, path))
    }
    /// Sends a request and deserializes the json-response into the given type.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// This method will return a [TeatimeError] if the request fails or the response cannot be
    /// deserialized into the given type.
    /// NOTE: This method is not recommended for general use. Use the more specific methods
    /// provided by the [Client] struct if they exist.
    /// You are responsible for providing the correct Model for the response.
    pub async fn make_request<T: DeserializeOwned>(&self, req: reqwest::Request) -> Result<T> {
        self.cli
            .execute(req)
            .await?
            .json::<T>()
            .await
            .map_err(|e| e.into())
    }
}

/// Converts a [reqwest::Error] into a [TeatimeError].
/// This method exists for us to be able to directly call the unwrap operator (`?`) on the result
/// of a [reqwest::Result].
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
