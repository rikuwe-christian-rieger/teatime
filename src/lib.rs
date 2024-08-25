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
//! ```
//! # use teatime::{Client, Auth};
//! # async fn create_repo() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
//! let repo = client
//!     .repos()
//!     .create("my-new-repo")
//!     // Optional fields
//!     .description("This is my new repo".to_string())
//!     .private(true)
//!     // Send the request
//!     .send(&client)
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! Similarly, to get a list of commits for a repository, you can call:
//! ```
//! # use teatime::{Client, Auth};
//! # async fn get_commits() {
//! let client = Client::new("https://gitea.example.com", Auth::Token("your-token"));
//! let commits = client
//!    .repos()
//!    .get_commits("owner", "repo-name")
//!    // Optional fields
//!    .page(2)
//!    .send(&client)
//!    .await
//!    .unwrap();
//! # }
//! ```
//!
//! If you want to create a new access token for a user, you can call:
//! ```
//! # use teatime::{Client, CreateAccessTokenOption, Auth};
//! # async fn create_access_token() {
//! let basic = Auth::Basic("username", "password");
//! let client = Client::new("https://gitea.example.com", basic);
//! let token = client
//!     .user()
//!     .create_access_token("username", "my-new-token", vec!["write:repo"])
//!     .send(&client)
//!     .await
//!     .unwrap();
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

pub mod api;
pub mod model;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessTokenOption {
    /// Access token name.
    pub name: String,
    /// Optional scopes for the access token.
    pub scopes: Option<Vec<String>>,
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
    pub fn new(base_url: impl ToString, auth: Auth<impl ToString>) -> Self {
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
            .user_agent(format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .expect("client build error");

        Self {
            cli,
            base_url: base_url.to_string(),
        }
    }

    pub fn repos(&self) -> api::repos::Repos {
        api::repos::Repos
    }

    pub fn issues(&self) -> api::issues::Issues {
        api::issues::Issues
    }

    pub fn user(&self) -> api::user::User {
        api::user::User
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
