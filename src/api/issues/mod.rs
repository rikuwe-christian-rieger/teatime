pub mod create;
pub mod delete;
pub mod list;

pub struct Issues;

impl Issues {
    /// Create an issue.
    /// If using deadline only the date will be taken into account, and time of day ignored.
    /// The only required field in the [CreateIssueOption] is `title`. All other fields are
    /// optional.
    /// This method will return the created issue.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn create_issue() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let issue = client
    ///    .issues()
    ///    .create("owner", "repo", "my-new-issue")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will create a new issue with the title "my-new-issue" in the repository "owner/repo".
    pub fn create(
        &self,
        owner: impl ToString,
        repo: impl ToString,
        title: impl ToString,
    ) -> create::CreateIssueBuilder {
        create::CreateIssueBuilder::new(owner, repo, title)
    }
    /// Delete an issue.
    /// This will delete the issue with the given issue number.
    /// WARNING: This is irreversible and will not ask for confirmation. Use with caution.
    ///
    /// This method will return a 204 status code if the issue was successfully deleted.
    /// If the issue does not exist, this method will return a 404 status code.
    /// If the user is not authorized to delete the issue, this method will return a 403 status
    /// code.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn delete_issue() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///    .issues()
    ///    .delete("owner", "repo", 1)
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will delete the issue #1 in the repository "owner/repo".
    pub fn delete(
        &self,
        owner: impl ToString,
        repo: impl ToString,
        issue_number: i64,
    ) -> delete::DeleteIssueBuilder {
        delete::DeleteIssueBuilder::new(owner, repo, issue_number)
    }

    /// List a repository's issues.
    /// The [GetIssuesOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent issues for the repository.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # use teatime::{Client, Auth, model::issues::State};
    /// # async fn get_issues() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let issues = client
    ///   .issues()
    ///   .list("owner", "repo")
    ///   .state(State::Open)
    ///    .send(&client)
    ///   .await
    ///   .unwrap();
    /// # }
    /// ```
    /// This will get all open issues in the repository "owner/repo".
    pub fn list(&self, owner: impl ToString, repo: impl ToString) -> list::ListIssuesBuilder {
        list::ListIssuesBuilder::new(owner, repo)
    }
}
