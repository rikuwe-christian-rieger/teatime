pub mod commits;
pub mod delete;
pub mod edit;
pub mod forks;
pub mod get;

/// The [Repos] struct provides methods for interacting with repositories.
pub struct Repos {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

impl Repos {
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
    ///    .repos("owner", "repo")
    ///    .delete()
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    pub fn delete(&self) -> delete::DeleteRepoBuilder {
        delete::DeleteRepoBuilder::new(&self.owner, &self.repo)
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
    ///     .repos("owner", "repo")
    ///     .get()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    pub fn get(&self) -> get::GetRepoBuilder {
        get::GetRepoBuilder::new(&self.owner, &self.repo)
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
    ///     .repos("owner", "repo")
    ///     .create_fork()
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
    ///    .repos("owner", "repo")
    ///    .create_fork()
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
    ///   .user()
    ///   .create_repo("my-new-repo")
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// let forked_repo = client
    ///    .repos("owner", "my-new-repo")
    ///    .create_fork()
    ///    .name("my-new-repo".to_string())
    ///    .send(&client)
    ///    .await
    ///    .expect_err("Repository with the same name should already exist for the current user");
    /// # }
    /// ```
    /// This will create a new repository with the name "my-new-repo" for the authenticated user,
    /// then attempt to fork the repository "owner/repo" into the authenticated user's account.
    /// The fork will fail because a repository with the same name already exists.
    pub fn create_fork(&self) -> forks::CreateForkBuilder {
        forks::CreateForkBuilder::new(&self.owner, &self.repo)
    }

    /// Edits a repository by its owner and name.
    ///
    /// If you don't set any fields, the repository will not be modified.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn edit_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///      Auth::Token("your-token")
    /// );
    /// let repo = client
    ///     .repos("owner", "repo")
    ///     .edit()
    ///     .description("A new description".to_string())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will set the description of the repository "owner/repo" to "A new description".
    /// If you want to remove the description, you can set it to an empty string.
    pub fn edit(&self) -> edit::EditRepoBuilder {
        edit::EditRepoBuilder::new(&self.owner, &self.repo)
    }

    /// Lists the forks of a repository by its owner and name.
    pub fn get_forks(&self) -> forks::ListForksBuilder {
        forks::ListForksBuilder::new(&self.owner, &self.repo)
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
    ///   .repos("owner", "repo")
    ///   .get_commits()
    ///   .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    pub fn get_commits(&self) -> commits::GetCommitsBuilder {
        commits::GetCommitsBuilder::new(&self.owner, &self.repo)
    }
}
