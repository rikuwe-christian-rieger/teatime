//! This crate is a simple Gitea API client. It's goal is to give you the ability to write
//! exactly as much code as you need to interact with the specific parts of the Gitea API you need,
//! but no more.
//!
//! # Usage
//! The main way to interact with the Gitea API is through the `Client` struct. You can create a
//! new [Client] by calling [Client::new] with the base URL of your Gitea instance and a personal
//! token. Teatime does currently not support basic HTML or OAuth2 authentication.
//!
//! Once you have obtained a [Client], you can interact with the Gitea API by calling the various
//! methods the instance provides. For example, to create a new repository, you can call:
//! ```rust
//! # use teatime::{Client, CreateRepoOption, Auth};
//! # async fn create_repo() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
//! let create_option = CreateRepoOption {
//!     // `name` is the only required field
//!     name: "my-new-repo".to_string(),
//!     ..Default::default()
//! };
//! // This will create a new repository with the name "my-new-repo" for the authenticated user.
//! let repo = client.user_create_repository(&create_option).await.unwrap();
//! # }
//! ```
//!
//! Similarly, to get a list of commits for a repository, you can call:
//! ```rust
//! # use teatime::{Client, GetCommitsOption, Auth};
//! # async fn get_commits() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
//! let get_option = GetCommitsOption {
//!     // `GetCommitsOption` has a number of optional fields to filter the results,
//!     // but none are required. In this example, we're just setting the `limit` to 10 to
//!     // only get the 10 most recent commits.
//!     limit: Some(10),
//!     ..Default::default()
//! };
//! let commits = client.get_commits("owner", "repo", &get_option).await.unwrap();
//! # }
//! ```
//!
//! If you want to create a new access token for a user, you can call:
//! ```rust
//! # use teatime::{Client, CreateAccessTokenOption, Auth};
//! # async fn create_access_token() {
//! let basic = Auth::Basic("username", "password");
//! let client = Client::new("https://gitea.example.com", basic);
//! let create_option = CreateAccessTokenOption {
//!    name: "my-new-token".to_string(),
//!    ..Default::default()
//! };
//! let token = client.create_access_token("username", &create_option).await.unwrap();
//! println!("Token {} created: {}", token.name, token.sha1);
//! // You can now create a new client with the token and use it to interact with the API.
//! let new_client = Client::new("https://gitea.example.com", Auth::Token(token.sha1));
//! # }
//!
//!
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use base64::{alphabet, Engine};
use error::{Result, TeatimeError};
use std::fmt::Display;

use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Method, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod error;

mod issue;
mod repo;
pub use issue::*;
pub use repo::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessTokenOption {
    /// Access token name.
    pub name: String,
    /// Optional scopes for the access token.
    pub scopes: Option<Vec<String>>,
}

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

/// Represents the authentication method to use with the Gitea API.
pub enum Auth<D: ToString> {
    Token(D),
    Basic(D, D),
    None,
}

/// Represents a Gitea client.
/// This struct is the main way to interact with the Gitea API.
/// It provides methods for creating repositories, getting repositories, deleting repositories,
/// and listing a repo's commits.
pub struct Client {
    cli: reqwest::Client,
    base_url: String,
}

impl Client {
    /// Creates a new Gitea client with the given base URL and personal token.
    /// NOTE: The base URL MUST not include the `/api/v1` path and should not contain any trailing
    /// slashes. For example, `https://gitea.example.com` is a valid base URL, but
    /// `https://gitea.example.com/` or `https://gitea.example.com/api/v1` are not.
    pub fn new<A: ToString, B: ToString>(base_url: A, auth: Auth<B>) -> Self {
        let mut headers = HeaderMap::new();
        match auth {
            Auth::Token(token) => {
                let token = HeaderValue::from_str(&format!("token {}", token.to_string()))
                    .expect("token error");
                headers.insert(header::AUTHORIZATION, token);
            }
            Auth::Basic(user, pass) => {
                let engine = GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::new());
                let base = engine.encode(format!("{}:{}", user.to_string(), pass.to_string()));
                let basic =
                    HeaderValue::from_str(&format!("Basic {base}")).expect("basic auth error");
                headers.insert(header::AUTHORIZATION, basic);
            }
            Auth::None => {}
        };
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let cli = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .user_agent("teatime/0.0.1")
            .build()
            .expect("client build error");

        Self {
            cli,
            base_url: base_url.to_string(),
        }
    }

    /// Gets the currently authenticated user.
    /// This will return a [User] object representing the currently authenticated user.
    /// As long as the token is valid, this method will always return a user.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_authenticated_user() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("6adb63fdb8fcfa101207281cdf5e1d28b125e9ec"));
    /// let user = client.get_authenticated_user().await.unwrap();
    /// # }
    pub async fn get_authenticated_user(&self) -> Result<User> {
        let req = self.get("user").build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Gets a user by their username.
    /// This will return a [User] object if the user exists and is visible to the currently
    /// authenticated user.
    /// If the user does not exist or is not visible, this method will return a 404 status code and
    /// an empty response.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn get_user() {
    /// let client = Client::new("https://gitea.example.com",
    /// Auth::Token("5fda63fdbbfcfd131607881cda5e1d28a215e9e1"));
    /// let user = client.get_user("username
    /// ").await.unwrap();
    /// # }
    /// ```
    /// This will get the user with the username "username".
    /// If the user does not exist, this method will return a [TeatimeError] with a 404 status code.
    ///
    pub async fn get_user<A: ToString>(&self, username: A) -> Result<User> {
        let req = self
            .get(format!("users/{}", username.to_string()))
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Creates a new access token for a user.
    /// Technically, only the `name` field is required, but it's recommended to set the `scopes`
    /// since the token will usually have no permissions without them (which can lead to an error
    /// on gitea's side despite the token being created).
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, CreateAccessTokenOption, Auth};
    /// # async fn create_token() {
    /// let client = Client::new("https://gitea.example.com", Auth::Basic("username", "password"));
    /// let options = CreateAccessTokenOption {
    ///   name: "my-new-token".to_string(),
    ///   scopes: Some(vec!["read:user".to_string(), "write:repository".to_string()]),
    /// };
    /// let token = client.create_access_token("username", &options).await.unwrap();
    /// println!("Token {} created: {}", token.name, token.sha1);
    /// let new_client = Client::new("https://gitea.example.com", Auth::Token(token.sha1));
    /// # }
    /// ```
    /// This will create a new token with the name "my-new-token", which can read all user data and
    /// read and write to repositories.
    ///
    /// If the token is successfully created, this method will return a [AccessToken] object.
    /// If the user is not authenticated correctly (e.g. not using basic auth), this method will
    /// return a 403 status code.
    /// In case of any client-side errors, this method will return a 400 status code.
    pub async fn create_access_token<A: ToString>(
        &self,
        username: A,
        options: &CreateAccessTokenOption,
    ) -> Result<AccessToken> {
        let username = username.to_string();
        let req = self
            .post(format!("users/{username}/tokens"))
            .json(options)
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Deletes an access token by its username and token.
    /// This will delete the token and revoke all permissions associated with it.
    /// NOTE: This endpoint requires basic authentication and will fail otherwise.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn delete_token() {
    /// let client = Client::new("https://gitea.example.com", Auth::Basic("username", "password"));
    /// client.delete_access_token("username", "token-name").await.unwrap();
    /// # }
    /// ```
    /// This will delete the token with the name "token-name" for the user "username".
    ///
    /// If the token does not exist, this method will return a 404 status code.
    /// If the target user is not the authenticated user and the authenticated user is not an
    /// administrator, this method will return a 403 status code.
    /// For any client-side other errors, this method will return a 422 status code.
    /// If the token is successfully deleted, this method will return a 204 status code.
    pub async fn delete_access_token<A: ToString, B: ToString>(
        &self,
        username: A,
        token: B,
    ) -> Result<()> {
        let username = username.to_string();
        let token = token.to_string();
        let req = self
            .delete(format!("users/{username}/tokens/{token}"))
            .build()?;
        self.make_request(req).await?;
        Ok(())
    }

    /// Creates a new DELETE-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn delete(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::DELETE, path)
    }
    /// Creates a new POST-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn post(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::POST, path)
    }
    /// Creates a new POST-request builder with the given path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn get(&self, path: impl Display) -> reqwest::RequestBuilder {
        self.request_base(Method::GET, path)
    }
    /// Creates a new request builder with the given method and path.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// `path` will be prefixed with `{base_url}/api/v1/` before the request is sent.
    pub fn request_base(&self, method: Method, path: impl Display) -> reqwest::RequestBuilder {
        self.cli
            .request(method, format!("{}/api/v1/{}", self.base_url, path))
    }
    /// Sends a request and checks the response for errors.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// This method will return a [TeatimeError] if the request fails.
    /// /// NOTE: This method is not recommended for general use. Use the more specific methods
    /// provided by the [Client] struct if they exist.
    /// You are responsible for providing the correct Model for the response.
    pub async fn make_request(&self, req: reqwest::Request) -> Result<Response> {
        let res = self.cli.execute(req).await?;
        let status = res.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(TeatimeError {
                message: res.text().await.unwrap_or_default(),
                kind: error::TeatimeErrorKind::HttpError,
                status_code: status,
            });
        }
        Ok(res)
    }
    /// Parses a json response into a given model.
    /// You may use this method to talk to the Gitea API directly if you need to.
    /// This method will return a [TeatimeError] if the response cannot be deserialized.
    /// /// NOTE: This method is not recommended for general use. Use the more specific methods
    /// provided by the [Client] struct if they exist.
    /// You are responsible for providing the correct Model for the response.
    pub async fn parse_response<T: DeserializeOwned>(&self, res: reqwest::Response) -> Result<T> {
        let status_code = res.status();
        let text = res.text().await?;
        serde_json::from_str(&text).map_err(|e| TeatimeError {
            message: format!("Error parsing response: {}", e),
            kind: error::TeatimeErrorKind::SerializationError,
            status_code,
        })
    }
}

/// NOTE: This is a workaround for the janky `#[serde(default)]` attribute.
/// It's not possible to use `#[serde(default = true)]`, so we have to create this
/// helper function and use `#[serde(default = "default_true")]` instead.
/// This is a known issue: #368 (https://github.com/serde-rs/serde/issues/368)
pub const fn default_true() -> bool {
    true
}
