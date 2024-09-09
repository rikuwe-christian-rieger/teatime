use build_it::Builder;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

#[derive(Debug, Clone, Builder, Serialize)]
#[build_it(into)]
pub struct ListPublicMembersBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    org: String,
    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results.
    limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct IsPublicMemberBuilder {
    org: String,
    username: String,
}

#[derive(Debug, Clone)]
pub struct ConcealMembershipBuilder {
    org: String,
    username: String,
}

#[derive(Debug, Clone)]
pub struct PublicizeMembershipBuilder {
    org: String,
    username: String,
}

impl ListPublicMembersBuilder {
    pub fn new(org: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list an organization's public members.
    /// This will return a list of [User] objects.
    pub async fn send(&self, client: &Client) -> Result<Vec<User>> {
        let req = client
            .get(format!("/orgs/{}/public_members", self.org))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl IsPublicMemberBuilder {
    pub fn new(org: impl ToString, username: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            username: username.to_string(),
        }
    }
    /// Sends the request to check if a user is a public member of an organization.
    pub async fn send(&self, client: &Client) -> Result<bool> {
        let Self { org, username } = self;
        let req = client
            .get(format!("/orgs/{org}/public_members/{username}"))
            .build()?;
        match client.make_request(req).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.status_code == StatusCode::NOT_FOUND {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }
}

impl ConcealMembershipBuilder {
    pub fn new(org: impl ToString, username: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            username: username.to_string(),
        }
    }
    /// Sends the request to conceal a user's membership in an organization.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self { org, username } = self;
        let req = client
            .delete(format!("/orgs/{org}/public_members/{username}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}

impl PublicizeMembershipBuilder {
    pub fn new(org: impl ToString, username: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            username: username.to_string(),
        }
    }
    /// Sends the request to publicize a user's membership in an organization.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self { org, username } = self;
        let req = client
            .put(format!("/orgs/{org}/public_members/{username}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}
