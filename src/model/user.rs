use serde::{Deserialize, Serialize};

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
