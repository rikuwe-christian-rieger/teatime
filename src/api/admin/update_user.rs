use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

/// Represents the options for creating a new user.
/// The only required field is `email` and `username`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct UpdateUserBuilder {
    #[build_it(skip)]
    #[serde(skip)]
    /// the username of the user
    username: String,
    #[build_it(skip)]
    /// The source id
    pub source_id: i64,
    #[build_it(skip)]
    /// The user's authenticated sign-in name. Empty by default.
    pub login_name: String,
    /// Whether user is admin
    pub admin: Option<bool>,
    /// Whether user is allowdd to create organizations
    pub allow_create_organization: Option<bool>,
    /// Whether user is allowdd to create git hooks
    pub allow_git_hook: Option<bool>,
    /// Whether user is allowdd to import
    pub allow_import_local: Option<bool>,
    /// Description of the user
    pub description: Option<String>,
    /// Email of the user
    pub email: Option<String>,
    /// Location of the user
    pub location: Option<String>,
    /// Number of repos the user is allowed to create
    pub max_repo_creation: Option<i64>,
    /// Whether the user is allowed to login
    pub prohibit_login: Option<bool>,
    /// Website of the user
    pub website: Option<String>,
    /// Full name of the user.
    pub full_name: Option<String>,
    /// If the user needs to change the password.
    pub must_change_password: Option<String>,
    /// The password of the user
    pub password: Option<String>,
    /// Whether the user is restricted.
    pub restricted: Option<bool>,
    /// User visibility.
    /// Can be one of "public", "limited", or "private".
    pub visibility: Option<String>,
}

impl UpdateUserBuilder {
    pub fn new(username: impl ToString, login_name: impl ToString, source_id: i64) -> Self {
        Self {
            username: username.to_string(),
            source_id,
            login_name: login_name.to_string(),
            admin: None,
            allow_create_organization: None,
            allow_git_hook: None,
            allow_import_local: None,
            description: None,
            email: None,
            location: None,
            max_repo_creation: None,
            prohibit_login: None,
            website: None,
            full_name: None,
            must_change_password: None,
            password: None,
            restricted: None,
            visibility: None,
        }
    }
    /// Send the request to update the user.
    /// This will return the updated [User].
    pub async fn send(&self, client: &Client) -> Result<User> {
        let username = &self.username;
        let req = client
            .patch(format!("admin/users/{username}"))
            .json(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
