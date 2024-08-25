use build_it::Builder;
use serde::{Deserialize, Serialize};

use crate::model::repos::Repository;
use crate::error::Result;

/// Options for searching repositories.
/// All fields are optional.
#[derive(Default, Debug, Clone, Serialize, Builder)]
pub struct SearchRepositoriesBuilder {
    /// Keyword to search for
    pub q: Option<String>,
    /// Limit search to repositories with keyword as topic
    pub topic: Option<bool>,
    /// Include search of keyword within repository description
    #[serde(rename = "include_desc")]
    pub include_desc: Option<bool>,
    /// Search only for repos that the user with the given id owns or contributes to
    pub uid: Option<i64>,
    /// Repo owner to prioritize in the results
    pub priority_owner_id: Option<i64>,
    /// Search only for repos that belong to the given team id
    pub team_id: Option<i64>,
    /// Search only for repos that the user with the given id has starred
    #[serde(rename = "starredBy")]
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
        {
            let mut params = req.url_mut().query_pairs_mut();

            if let Some(q) = &self.q {
                params.append_pair("q", q);
            }
            if let Some(topic) = &self.topic {
                params.append_pair("topic", &topic.to_string());
            }
            if let Some(include_desc) = &self.include_desc {
                params.append_pair("include_desc", &include_desc.to_string());
            }
            if let Some(uid) = &self.uid {
                params.append_pair("uid", &uid.to_string());
            }
            if let Some(priority_owner_id) = &self.priority_owner_id {
                params.append_pair("priority_owner_id", &priority_owner_id.to_string());
            }
            if let Some(team_id) = &self.team_id {
                params.append_pair("team_id", &team_id.to_string());
            }
            if let Some(starred_by) = &self.starred_by {
                params.append_pair("starredBy", &starred_by.to_string());
            }
            if let Some(private) = &self.private {
                params.append_pair("private", &private.to_string());
            }
            if let Some(is_private) = &self.is_private {
                params.append_pair("is_private", &is_private.to_string());
            }
            if let Some(template) = &self.template {
                params.append_pair("template", &template.to_string());
            }
            if let Some(archived) = &self.archived {
                params.append_pair("archived", &archived.to_string());
            }
            if let Some(mode) = &self.mode {
                params.append_pair("mode", mode);
            }
            if let Some(exclusive) = &self.exclusive {
                params.append_pair("exclusive", &exclusive.to_string());
            }
            if let Some(sort) = &self.sort {
                params.append_pair("sort", sort);
            }
            if let Some(order) = &self.order {
                params.append_pair("order", order);
            }
            if let Some(page) = &self.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &self.limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
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
