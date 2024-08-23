use serde::{Deserialize, Serialize};

use crate::{default_true, error::Result, Client, User};

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

/// Options for forking a repository.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateForkOption {
    /// The name of the new repository.
    /// Will be the same as the original if not set.
    pub name: Option<String>,
    /// Organization name, if forking into an organization.
    pub organization: Option<String>,
}

/// Options for listing a repository's forks.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListForksOption {
    /// Optional page number of the results to fetch (1-based).
    /// Defaults to 1 if not set.
    pub page: Option<i64>,
    /// Optional number of forks to return per page (page-size).
    /// Defaults to the maximum your instance allows if not set.
    pub limit: Option<i64>,
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

impl Client {
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

    /// Forks a repository by its owner and name.
    /// The [CreateForkOption] struct provides a number of optional fields to customize the fork,
    /// but all fields are entirely optional.
    /// If you don't set any fields, the fork will be created with the same name as the original
    /// repository in the authenticated user's account.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use teatime::{Client, CreateForkOption, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new("https://gitea.example.com",
    ///     Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let fork_option = CreateForkOption {
    ///     name: Some("my-fork".to_string()),
    ///     ..Default::default()
    /// };
    /// let forked_repo = client.fork_repository("owner", "repo", &fork_option).await.unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the authenticated user's account with the
    /// name "my-fork".
    ///
    /// ```rust
    /// # use teatime::{Client, CreateForkOption, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new("https://gitea.example.com",
    ///     Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let fork_option = CreateForkOption {
    ///    organization: Some("my-org".to_string()),
    ///    ..Default::default()
    /// };
    /// let forked_repo = client.fork_repository("owner", "repo", &fork_option).await.unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the organization "my-org" with the same
    /// name as the original repository.
    ///
    /// ```rust
    /// # use teatime::{Client, CreateRepoOption, CreateForkOption, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new("https://gitea.example.com",
    ///     Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let create_option = CreateRepoOption {
    ///     name: "my-new-repo".to_string(),
    ///     ..Default::default()
    /// };
    /// let fork_option = CreateForkOption {
    ///     name: Some("my-new-repo".to_string()),
    ///     ..Default::default()
    /// };
    /// let created_repo = client.user_create_repository(&create_option).await.unwrap();
    /// let forked_repo = client
    ///     .fork_repository("owner", "repo", &fork_option)
    ///     .await
    ///     .expect_err("Repository with the same name should already exist for the current user");
    /// # }
    /// ```
    /// This will create a new repository with the name "my-new-repo" for the authenticated user,
    /// then attempt to fork the repository "owner/repo" into the authenticated user's account.
    /// The fork will fail because a repository with the same name already exists.
    pub async fn fork_repository<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        fork_option: &CreateForkOption,
    ) -> Result<Repository> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let req = self
            .post(format!("repos/{owner}/{repo}/forks"))
            .json(fork_option)
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Lists the forks of a repository by its owner and name.
    /// The [ListForksOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the first page of forks.
    ///
    pub async fn get_forks<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        list_option: &ListForksOption,
    ) -> Result<Vec<Repository>> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let mut req = self.get(format!("repos/{owner}/{repo}/forks")).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(page) = &list_option.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &list_option.limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
        let res = self.make_request(req).await?;
        self.parse_response(res).await
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
}
