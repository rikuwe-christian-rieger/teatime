pub mod create_repo;
pub mod current;
pub mod get;
pub mod list_repos;
pub mod tokens;

pub struct User;

impl User {
    /// Gets the currently authenticated user.
    /// This will return a [User] object representing the currently authenticated user.
    /// As long as the token is valid, this method will always return a user.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn get_authenticated_user() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let user = client
    ///     .user()
    ///     .current()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    pub fn current(&self) -> current::GetAuthenticatedUserBuilder {
        current::GetAuthenticatedUserBuilder::new()
    }

    /// Gets a user by their username.
    /// This will return a [User] object if the user exists and is visible to the currently
    /// authenticated user.
    /// If the user does not exist or is not visible, this method will return a 404 status code and
    /// an empty response.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn get_user() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let user = client
    ///    .user()
    ///    .get("username")
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will get the user with the username "username".
    /// If the user does not exist, this method will return a [TeatimeError] with a 404 status code.
    ///
    pub fn get(&self, username: impl ToString) -> get::GetUserBuilder {
        get::GetUserBuilder::new(username)
    }

    /// Creates a new repository for the authenticated user.
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
    ///     .user()
    ///     .create_repo("my-new-repo")
    ///     .private(true)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new private repository with the name "my-new-repo" for the authenticated user.
    pub fn create_repo(&self, name: impl ToString) -> create_repo::CreateRepoBuilder {
        create_repo::CreateRepoBuilder::new(name)
    }

    /// Lists all repositories for the authenticated user.
    /// This will return a list of all [Repository](crate::model::repos::Repository) objects
    /// owned by the authenticated user.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn list_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repos = client
    ///     .user()
    ///     .list_repos()
    ///     .limit(10)
    ///     .page(2)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_repos(&self) -> list_repos::ListReposBuilder {
        list_repos::ListReposBuilder::new()
    }

    /// Creates a new access token for a user.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, CreateAccessTokenOption, Auth};
    /// # async fn create_token() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Basic("username", "password")
    /// );
    /// let token = client
    ///     .user()
    ///     .create_access_token("username", "my-new-token", vec!["write:repository", "read:user"])
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// println!("Token {} created: {}", token.name, token.sha1);
    /// let new_client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token(token.sha1)
    /// );
    /// # }
    /// ```
    /// This will create a new token with the name "my-new-token", which can read all user data and
    /// read and write to repositories.
    ///
    /// If the token is successfully created, this method will return a [AccessToken] object.
    /// If the user is not authenticated correctly (e.g. not using basic auth), this method will
    /// return a 403 status code.
    /// In case of any client-side errors, this method will return a 400 status code.
    pub fn create_access_token(
        &self,
        user: impl ToString,
        name: impl ToString,
        scopes: Vec<impl ToString>,
    ) -> tokens::CreateAccessTokenBuilder {
        tokens::CreateAccessTokenBuilder::new(user, name, scopes)
    }

    /// Deletes an access token by its username and token.
    /// This will delete the token and revoke all permissions associated with it.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```
    /// # use teatime::{Client, Auth};
    /// # async fn delete_token() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Basic("username", "password")
    /// );
    /// client.
    ///     user()
    ///     .delete_access_token("username", "token")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will delete the token with the name "token-name" for the user "username".
    ///
    /// If the token does not exist, this method will return a 404 status code.
    /// If the target user is not the authenticated user and the authenticated user is not an
    /// administrator, this method will return a 403 status code.
    /// For any client-side other errors, this method will return a 422 status code.
    /// If the token is successfully deleted, this method will return a 204 status code.
    pub fn delete_access_token(
        &self,
        user: impl ToString,
        token: impl ToString,
    ) -> tokens::DeleteAccessTokenBuilder {
        tokens::DeleteAccessTokenBuilder::new(user, token)
    }
}
