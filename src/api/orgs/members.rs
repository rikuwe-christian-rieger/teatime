use build_it::Builder;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{error::Result, model::user::User, Client};

#[derive(Debug, Clone, Builder, Serialize)]
#[build_it(into)]
pub struct ListMembersBuilder {
    #[serde(skip)]
    #[build_it(skip)]
    org: String,
    /// Page number of results to return (1-based).
    page: Option<i64>,
    /// Page size of results.
    limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct IsMemberBuilder {
    org: String,
    username: String,
}

#[derive(Debug, Clone)]
pub struct RemoveMemberBuilder {
    org: String,
    username: String,
}

impl ListMembersBuilder {
    pub fn new(org: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            page: None,
            limit: None,
        }
    }
    /// Sends the request to list an organization's members.
    /// This will return a list of [User] objects.
    pub async fn send(&self, client: &Client) -> Result<Vec<User>> {
        let req = client
            .get(format!("/orgs/{}/members", self.org))
            .query(self)
            .build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl IsMemberBuilder {
    pub fn new(org: impl ToString, username: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            username: username.to_string(),
        }
    }
    /// Sends the request to check if a user is a member of an organization.
    pub async fn send(&self, client: &Client) -> Result<bool> {
        let Self { org, username } = self;
        let req = client
            .get(format!("/orgs/{org}/members/{username}"))
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

impl RemoveMemberBuilder {
    pub fn new(org: impl ToString, username: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            username: username.to_string(),
        }
    }
    /// Sends the request to remove a user from an organization.
    pub async fn send(&self, client: &Client) -> Result<()> {
        let Self { org, username } = self;
        let req = client
            .delete(format!("/orgs/{org}/members/{username}"))
            .build()?;
        let _ = client.make_request(req).await?;
        Ok(())
    }
}
