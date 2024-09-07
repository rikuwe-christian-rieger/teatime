pub mod get;
pub mod orgs;
pub mod repos;
pub mod stars;

pub struct Users {
    pub username: String,
}

impl Users {
    /// Gets a user by their username.
    /// This will return a [User] object if the user exists and is visible to the currently
    /// authenticated user.
    /// If the user does not exist or is not visible, this method will return a 404 status code and
    /// an empty response.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_user() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let user = client
    ///    .users("username")
    ///    .get()
    ///    .send(&client)
    ///    .await
    ///    .unwrap();
    /// # }
    /// ```
    /// This will get the user with the username "username".
    /// If the user does not exist, this method will return a [TeatimeError] with a 404 status code.
    ///
    pub fn get(&self) -> get::GetUserBuilder {
        get::GetUserBuilder::new(&self.username)
    }

    /// Gets the repositories for a user.
    /// This will return a list of repositories for the user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repos = client
    ///     .users("username")
    ///     .list_repos()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_repos(&self) -> repos::ListReposBuilder {
        repos::ListReposBuilder::new(&self.username)
    }

    /// Gets the stars for a user.
    /// This will return a list of starred repositories for the user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_stars() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let stars = client
    ///     .users("username")
    ///     .list_starred()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_starred(&self) -> stars::ListStarredBuilder {
        stars::ListStarredBuilder::new(&self.username)
    }

    /// Gets the organizations for a user.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_orgs() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let orgs = client
    ///     .users("username")
    ///     .list_orgs()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_orgs(&self) -> orgs::Orgs {
        orgs::Orgs::new(&self.username)
    }
}
