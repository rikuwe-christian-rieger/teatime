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
//! # use teatime::{Client, CreateRepoOption, Auth};
//! # async fn create_repo() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
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
//! # use teatime::{Client, GetCommitsOption, Auth};
//! # async fn get_commits() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
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
//! If you want to create a new access token for a user, you can call:
//! ```rust
//! # use teatime::{Client, CreateAccessTokenOption, Auth};
//! # async fn create_access_token() {
//! let basic = Auth::Basic("username", "password");
//! let client = Client::new("https://gitea.example.com", basic);
//! let create_option = CreateAccessTokenOption {
//!    name: "my-new-token".to_string(),
//!    ..Default::default()
//! };
//! let token = client.create_access_token("username", &create_option).await.unwrap();
//! println!("Token {} created: {}", token.name, token.sha1);
//! // You can now create a new client with the token and use it to interact with the API.
//! let new_client = Client::new("https://gitea.example.com", Auth::Token(token.sha1));
//! # }
//!
//!
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use base64::{alphabet, Engine};
use error::{Result, TeatimeError};
use std::fmt::Display;

use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Method, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod error;

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

/// Options for searching repositories.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SearchRepositoriesOption {
    /// Keyword to search for
    pub q: Option<String>,
    /// Limit search to repositories with keyword as topic
    pub topic: Option<bool>,
    /// Include search of keyword within repository description
    #[serde(rename = "include_desc")]
    pub include_desc: Option<bool>,
    /// Search only for repos that the user with the given id owns or contributes to
    pub uid: Option<i64>,
    /// Repo owner to prioritize in the results
    pub priority_owner_id: Option<i64>,
    /// Search only for repos that belong to the given team id
    pub team_id: Option<i64>,
    /// Search only for repos that the user with the given id has starred
    #[serde(rename = "starredBy")]
    pub starred_by: Option<i64>,
    /// Include private repositories this user has access to (defaults to true)
    pub private: Option<bool>,
    /// Show only pubic, private or all repositories (defaults to all)
    pub is_private: Option<bool>,
    /// Include template repositories this user has access to (defaults to true)
    pub template: Option<bool>,
    /// Show only archived, non-archived or all repositories (defaults to all)
    pub archived: Option<bool>,
    /// Type of repository to search for. Supported values are "fork", "source", "mirror" and "collaborative"
    pub mode: Option<String>,
    /// If uid is given, search only for repos that the user owns
    pub exclusive: Option<bool>,
    /// Sort repos by attribute. Supported values are "alpha", "created", "updated", "size", and "id". Default is "alpha"
    pub sort: Option<String>,
    /// Sort order, either "asc" (ascending) or "desc" (descending). Default is "asc", ignored if "sort" is not specified.
    pub order: Option<String>,
    /// Page number of results to return (1-based)
    pub page: Option<i32>,
    /// Page size of results
    pub limit: Option<i32>,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessTokenOption {
    /// Access token name.
    pub name: String,
    /// Optional scopes for the access token.
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// Represents an access token.
pub struct AccessToken {
    /// ID of the access token.
    pub id: i64,
    /// Name of the access token.
    pub name: String,
    /// The token's scopes.
    pub scopes: Option<Vec<String>>,
    /// The token's SHA1 hash. This is probably what you want to store to access the API.
    pub sha1: String,
    /// The token's last eight characters. Useful for verifying the token.
    pub token_last_eight: String,
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

/// Represents the authentication method to use with the Gitea API.
pub enum Auth<D: ToString> {
    Token(D),
    Basic(D, D),
    None,
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
    pub fn new<A: ToString, B: ToString>(base_url: A, auth: Auth<B>) -> Self {
        let mut headers = HeaderMap::new();
        match auth {
            Auth::Token(token) => {
                let token = HeaderValue::from_str(&format!("token {}", token.to_string()))
                    .expect("token error");
                headers.insert(header::AUTHORIZATION, token);
            }
            Auth::Basic(user, pass) => {
                let engine = GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::new());
                let base = engine.encode(format!("{}:{}", user.to_string(), pass.to_string()));
                let basic =
                    HeaderValue::from_str(&format!("Basic {base}")).expect("basic auth error");
                headers.insert(header::AUTHORIZATION, basic);
            }
            Auth::None => {}
        };
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let cli = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .user_agent("teatime/0.0.1")
            .build()
            .expect("client build error");

        Self {
            cli,
            base_url: base_url.to_string(),
        }
    }

    /// Gets the currently authenticated user.
    /// This will return a [User] object representing the currently authenticated user.
    /// As long as the token is valid, this method will always return a user.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_authenticated_user() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("6adb63fdb8fcfa101207281cdf5e1d28b125e9ec"));
    /// let user = client.get_authenticated_user().await.unwrap();
    /// # }
    pub async fn get_authenticated_user(&self) -> Result<User> {
        let req = self.get("user").build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Gets a user by their username.
    /// This will return a [User] object if the user exists and is visible to the currently
    /// authenticated user.
    /// If the user does not exist or is not visible, this method will return a 404 status code and
    /// an empty response.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_user() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("5fda63fdbbfcfd131607881cda5e1d28a215e9e1"));
    /// let user = client.get_user("username
    /// ").await.unwrap();
    /// # }
    /// ```
    /// This will get the user with the username "username".
    /// If the user does not exist, this method will return a [TeatimeError] with a 404 status code.
    ///
    pub async fn get_user<A: ToString>(&self, username: A) -> Result<User> {
        let req = self
            .get(format!("users/{}", username.to_string()))
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Creates a new repository for the authenticated user.
    /// The only required field in the [CreateRepoOption] is `name`.
    /// All other fields are optional.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, CreateRepoOption, Auth};
    /// # async fn create_repo() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("5fda63fdbbfcfd131607881cda5e1d28a215e9e1"));
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
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Gets a repository by its owner and name.
    /// This will return a [Repository] object if the repository exists and is visible to the
    /// currently authenticated user.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_repo() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("793eae2c1dcd71daf9e6cc0f8a448a39b45d3ff3"));
    /// let repo = client.get_repository("owner", "repo").await.unwrap();
    /// # }
    /// ```
    ///
    pub async fn get_repository<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
    ) -> Result<Repository> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let req = self.get(format!("repos/{owner}/{repo}")).build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Searches for repositories based on the given search options.
    /// All fields in the [SearchRepositoriesOption] are optional.
    /// This method will return a list of repositories that match the search criteria.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, SearchRepositoriesOption, Auth};
    /// # async fn search_repos() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let search_option = SearchRepositoriesOption {
    ///    q: Some("my-repo".to_string()),
    ///    ..Default::default()
    ///    };
    /// let repos = client.search_repositories(&search_option).await.unwrap();
    /// # }
    /// ```
    /// This will search for repositories matching the keyword "my-repo".
    /// The search will include the repository description and will return the first page of
    /// results.
    pub async fn search_repositories(
        &self,
        search_option: &SearchRepositoriesOption,
    ) -> Result<Vec<Repository>> {
        let mut req = self.get("repos/search".to_string()).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(q) = &search_option.q {
                params.append_pair("q", q);
            }
            if let Some(topic) = &search_option.topic {
                params.append_pair("topic", &topic.to_string());
            }
            if let Some(include_desc) = &search_option.include_desc {
                params.append_pair("include_desc", &include_desc.to_string());
            }
            if let Some(uid) = &search_option.uid {
                params.append_pair("uid", &uid.to_string());
            }
            if let Some(priority_owner_id) = &search_option.priority_owner_id {
                params.append_pair("priority_owner_id", &priority_owner_id.to_string());
            }
            if let Some(team_id) = &search_option.team_id {
                params.append_pair("team_id", &team_id.to_string());
            }
            if let Some(starred_by) = &search_option.starred_by {
                params.append_pair("starredBy", &starred_by.to_string());
            }
            if let Some(private) = &search_option.private {
                params.append_pair("private", &private.to_string());
            }
            if let Some(is_private) = &search_option.is_private {
                params.append_pair("is_private", &is_private.to_string());
            }
            if let Some(template) = &search_option.template {
                params.append_pair("template", &template.to_string());
            }
            if let Some(archived) = &search_option.archived {
                params.append_pair("archived", &archived.to_string());
            }
            if let Some(mode) = &search_option.mode {
                params.append_pair("mode", mode);
            }
            if let Some(exclusive) = &search_option.exclusive {
                params.append_pair("exclusive", &exclusive.to_string());
            }
            if let Some(sort) = &search_option.sort {
                params.append_pair("sort", sort);
            }
            if let Some(order) = &search_option.order {
                params.append_pair("order", order);
            }
            if let Some(page) = &search_option.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &search_option.limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
        #[derive(Deserialize)]
        struct Response {
            #[allow(dead_code)]
            ok: bool,
            data: Vec<Repository>,
        }
        let res = self.make_request(req).await?;
        Ok(self.parse_response::<Response>(res).await?.data)
    }

    /// Deletes a repository by its owner and name.
    /// WARNING: This is irreversible and will delete all data associated with the repository.
    /// This action cannot be undone. When invoking this method, you will not be asked for
    /// confirmation. Use with caution, please don't nuke your repositories.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn delete_repo() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// client.delete_repository("owner", "repo").await.unwrap();
    /// # }
    pub async fn delete_repository<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
    ) -> Result<()> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let req = self.delete(format!("repos/{owner}/{repo}")).build()?;
        self.make_request(req).await?;
        Ok(())
    }

    /// Gets a list of commits for a repository.
    /// The [GetCommitsOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent commits for the default branch.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, GetCommitsOption, Auth};
    /// # async fn get_commits() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let get_option = GetCommitsOption::default();
    /// let commits = client.get_commits("owner", "repo", &get_option).await.unwrap();
    /// # }
    pub async fn get_commits<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        get_option: &GetCommitsOption,
    ) -> Result<Vec<Commit>> {
        let owner = owner.to_string();
        let repo = repo.to_string();
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
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Creates a new access token for a user.
    /// Technically, only the `name` field is required, but it's recommended to set the `scopes`
    /// since the token will usually have no permissions without them (which can lead to an error
    /// on gitea's side despite the token being created).
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, CreateAccessTokenOption, Auth};
    /// # async fn create_token() {
    /// let client = Client::new("https://gitea.example.com", Auth::Basic("username", "password"));
    /// let options = CreateAccessTokenOption {
    ///   name: "my-new-token".to_string(),
    ///   scopes: Some(vec!["read:user".to_string(), "write:repository".to_string()]),
    /// };
    /// let token = client.create_access_token("username", &options).await.unwrap();
    /// println!("Token {} created: {}", token.name, token.sha1);
    /// let new_client = Client::new("https://gitea.example.com", Auth::Token(token.sha1));
    /// # }
    /// ```
    /// This will create a new token with the name "my-new-token", which can read all user data and
    /// read and write to repositories.
    ///
    /// If the token is successfully created, this method will return a [AccessToken] object.
    /// If the user is not authenticated correctly (e.g. not using basic auth), this method will
    /// return a 403 status code.
    /// In case of any client-side errors, this method will return a 400 status code.
    pub async fn create_access_token<A: ToString>(
        &self,
        username: A,
        options: &CreateAccessTokenOption,
    ) -> Result<AccessToken> {
        let username = username.to_string();
        let req = self
            .post(format!("users/{username}/tokens"))
            .json(options)
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Deletes an access token by its username and token.
    /// This will delete the token and revoke all permissions associated with it.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn delete_token() {
    /// let client = Client::new("https://gitea.example.com", Auth::Basic("username", "password"));
    /// client.delete_access_token("username", "token-name").await.unwrap();
    /// # }
    /// ```
    /// This will delete the token with the name "token-name" for the user "username".
    ///
    /// If the token does not exist, this method will return a 404 status code.
    /// If the target user is not the authenticated user and the authenticated user is not an
    /// administrator, this method will return a 403 status code.
    /// For any client-side other errors, this method will return a 422 status code.
    /// If the token is successfully deleted, this method will return a 204 status code.
    pub async fn delete_access_token<A: ToString, B: ToString>(
        &self,
        username: A,
        token: B,
    ) -> Result<()> {
        let username = username.to_string();
        let token = token.to_string();
        let req = self
            .delete(format!("users/{username}/tokens/{token}"))
            .build()?;
        self.make_request(req).await?;
        Ok(())
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
    /// Sends a request and checks the response for errors.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// This method will return a [TeatimeError] if the request fails.
    /// /// NOTE: This method is not recommended for general use. Use the more specific methods
    /// provided by the [Client] struct if they exist.
    /// You are responsible for providing the correct Model for the response.
    pub async fn make_request(&self, req: reqwest::Request) -> Result<Response> {
        let res = self.cli.execute(req).await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(TeatimeError {
                message: res.text().await.unwrap_or_default(),
                kind: error::TeatimeErrorKind::HttpError,
                status_code: status,
            });
        }
        Ok(res)
    }
    /// Parses a json response into a given model.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// This method will return a [TeatimeError] if the response cannot be deserialized.
    /// /// NOTE: This method is not recommended for general use. Use the more specific methods
    /// provided by the [Client] struct if they exist.
    /// You are responsible for providing the correct Model for the response.
    pub async fn parse_response<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        let status_code = res.status();
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| TeatimeError {
            message: format!("Error parsing response: {}", e),
            kind: error::TeatimeErrorKind::SerializationError,
            status_code,
        })
    }
}

/// NOTE: This is a workaround for the janky `#[serde(default)]` attribute.
/// It's not possible to use `#[serde(default = true)]`, so we have to create this
/// helper function and use `#[serde(default = "default_true")]` instead.
/// This is a known issue: #368 (https://github.com/serde-rs/serde/issues/368)
pub const fn default_true() -> bool {
    true
}
