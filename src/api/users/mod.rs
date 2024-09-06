pub mod get;

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
}
