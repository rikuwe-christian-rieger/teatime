use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

/// Represents the options for creating a new user.
/// The only required field is `email` and `username`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct CreateUserBuilder {
    #[build_it(skip)]
    /// Email of the user.
    pub email: String,
    #[build_it(skip)]
    /// Username of the user.
    pub username: String,
    #[build_it(skip)]
    /// The password of the user
    pub password: String,
    /// Date the user was created at.
    pub created_at: Option<String>,
    /// Full name of the user.
    pub full_name: Option<String>,
    /// If the user needs to change the password.
    pub must_change_password: Option<String>,
    /// Whether the user is restricted.
    pub restricted: Option<bool>,
    /// Whether to send notifications
    pub send_notify: Option<bool>,
    /// The source id
    pub source_id: Option<i64>,
    /// User visibility.
    /// Can be one of "public", "limited", or "private".
    pub visibility: Option<String>,
    /// The user's authenticated sign-in name. Empty by default.
    pub login_name: Option<String>,
}

impl CreateUserBuilder {
    pub fn new(email: impl ToString, username: impl ToString, password: impl ToString) -> Self {
        Self {
            email: email.to_string(),
            username: username.to_string(),
            created_at: None,
            full_name: None,
            must_change_password: None,
            password: password.to_string(),
            restricted: None,
            send_notify: None,
            source_id: None,
            visibility: None,
            login_name: None,
        }
    }
    /// Send the request to create the user.
    /// This will return the created [User].
    pub async fn send(&self, client: &Client) -> Result<User> {
        let req = client.post("admin/users".to_string()).json(self).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
