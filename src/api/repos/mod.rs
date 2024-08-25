pub mod commits;
pub mod create;
pub mod delete;
pub mod forks;
pub mod get;
pub mod search;

/// The [Repos] struct provides methods for interacting with repositories.
pub struct Repos;

impl Repos {
    /// Creates a new repository for the authenticated user.
    /// The only required field in the [CreateRepoOption] is `name`.
    /// All other fields are optional.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn create_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos()
    ///     .create("my-new-repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new repository with the name "my-new-repo" for the authenticated user.
    pub fn create(&self, name: impl ToString) -> create::CreateRepoBuilder {
        create::CreateRepoBuilder::new(name)
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
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// client
    ///    .repos()
    ///    .delete("owner", "repo")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn delete(
        &self,
        owner: impl ToString,
        repo: impl ToString,
    ) -> delete::DeleteRepoBuilder {
        delete::DeleteRepoBuilder::new(owner, repo)
    }
    /// Searches for repositories based on the given search options.
    /// All fields in the [SearchRepositoriesOption] are optional.
    /// This method will return a list of repositories that match the search criteria.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn search_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos()
    ///     .search()
    ///     .q("my-repo".to_string())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will search for repositories matching the keyword "my-repo".
    /// The search will include the repository description and will return the first page of
    /// results.
    pub fn search(&self) -> search::SearchRepositoriesBuilder {
        search::SearchRepositoriesBuilder::new()
    }
    /// Gets a repository by its owner and name.
    /// This will return a [Repository] object if the repository exists and is visible to the
    /// currently authenticated user.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos()
    ///     .get("owner", "repo")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    pub fn get(&self, owner: impl ToString, repo: impl ToString) -> get::GetRepoBuilder {
        get::GetRepoBuilder::new(owner, repo)
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
    /// # use teatime::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let forked_repo = client
    ///     .repos()
    ///     .create_fork("owner", "repo")
    ///     .name("my-fork".to_string())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the authenticated user's account with the
    /// name "my-fork".
    ///
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let forked_repo = client
    ///    .repos()
    ///    .create_fork("owner", "repo")
    ///    .organization("my-org".to_string())
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will fork the repository "owner/repo" into the organization "my-org" with the same
    /// name as the original repository.
    ///
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn fork_repo() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let created_repo = client
    ///   .repos()
    ///   .create("my-new-repo")
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// let forked_repo = client
    ///    .repos()
    ///    .create_fork("owner", "repo")
    ///    .name("my-new-repo".to_string())
    ///    .send(&client)
    ///    .await
    ///    .expect_err("Repository with the same name should already exist for the current user");
    /// # }
    /// ```
    /// This will create a new repository with the name "my-new-repo" for the authenticated user,
    /// then attempt to fork the repository "owner/repo" into the authenticated user's account.
    /// The fork will fail because a repository with the same name already exists.
    pub fn create_fork(
        &self,
        owner: impl ToString,
        repo: impl ToString,
    ) -> forks::CreateForkBuilder {
        forks::CreateForkBuilder::new(owner, repo)
    }

    /// Lists the forks of a repository by its owner and name.
    pub fn get_forks(&self, owner: impl ToString, repo: impl ToString) -> forks::ListForksBuilder {
        forks::ListForksBuilder::new(owner, repo)
    }

    /// Gets a list of commits for a repository.
    /// The [GetCommitsOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent commits for the default branch.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, api::repos::commits::GetCommitsBuilder, Auth};
    /// # async fn get_commits() {
    /// let client = Client::new(
    ///    "https://gitea.example.com",
    ///    Auth::Token("your-token")
    /// );
    /// let commits = client
    ///   .repos()
    ///   .get_commits("owner", "repo")
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    pub fn get_commits(
        &self,
        owner: impl ToString,
        repo: impl ToString,
    ) -> commits::GetCommitsBuilder {
        commits::GetCommitsBuilder::new(owner, repo)
    }
}
