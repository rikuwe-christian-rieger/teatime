use build_it::Builder;
use serde::{Deserialize, Serialize};
use teatime_macros::QueryParams;

use crate::error::Result;
use crate::model::repos::Repository;

/// Options for searching repositories.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Builder, QueryParams)]
pub struct SearchRepositoriesBuilder {
    /// Keyword to search for
    #[query_params(rename = "q")]
    pub query: Option<String>,
    /// Limit search to repositories with keyword as topic
    pub topic: Option<bool>,
    /// Include search of keyword within repository description
    #[query_params(rename = "includeDesc")]
    pub include_desc: Option<bool>,
    /// Search only for repos that the user with the given id owns or contributes to
    pub uid: Option<i64>,
    /// Repo owner to prioritize in the results
    pub priority_owner_id: Option<i64>,
    /// Search only for repos that belong to the given team id
    pub team_id: Option<i64>,
    /// Search only for repos that the user with the given id has starred
    #[query_params(rename = "starredBy")]
    pub starred_by: Option<i64>,
    /// Include private repositories this user has access to (defaults to true)
    pub private: Option<bool>,
    /// Show only pubic, private or all repositories (defaults to all)
    pub is_private: Option<bool>,
    /// Include template repositories this user has access to (defaults to true)
    pub template: Option<bool>,
    /// Show only archived, non-archived or all repositories (defaults to all)
    pub archived: Option<bool>,
    /// Type of repository to search for. Supported values are "fork", "source", "mirror" and "collaborative"
    pub mode: Option<String>,
    /// If uid is given, search only for repos that the user owns
    pub exclusive: Option<bool>,
    /// Sort repos by attribute. Supported values are "alpha", "created", "updated", "size", and "id". Default is "alpha"
    pub sort: Option<String>,
    /// Sort order, either "asc" (ascending) or "desc" (descending). Default is "asc", ignored if "sort" is not specified.
    pub order: Option<String>,
    /// Page number of results to return (1-based)
    pub page: Option<i32>,
    /// Page size of results
    pub limit: Option<i32>,
}

impl SearchRepositoriesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn send(&self, client: &crate::Client) -> Result<Vec<Repository>> {
        let mut req = client.get("repos/search".to_string()).build()?;
        self.append_query_params(&mut req);
        #[derive(Deserialize)]
        struct Response {
            #[allow(dead_code)]
            ok: bool,
            data: Vec<Repository>,
        }
        let res = client.make_request(req).await?;
        Ok(client.parse_response::<Response>(res).await?.data)
    }
}
