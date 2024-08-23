use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

use crate::{Client, User, error::Result};

/// Represents an attachment.
/// Attachments are used in issues, pull requests, and releases.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Attachment {
    pub browser_download_url: String,
    pub created_at: String,
    pub download_count: i64,
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub uuid: String,
}

/// Represents a label.
/// Labels are used in issues and pull requests.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Label {
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    pub id: i64,
    pub is_archived: bool,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// Represents the state of an issue.
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "all")]
    #[default]
    All,
}
impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Open => write!(f, "open"),
            State::Closed => write!(f, "closed"),
            State::All => write!(f, "all"),
        }
    }
}

/// Represents an issue in a repository.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Issue {
    pub assets: Vec<Attachment>,
    pub assignee: Option<User>,
    pub assignees: Option<Vec<User>>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub created_at: String,
    pub due_date: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub is_locked: bool,
    pub labels: Vec<Label>,
    pub number: i64,
    pub original_author: String,
    pub original_author_id: i64,
    pub pin_order: i64,
    pub r#ref: String,
    pub state: State,
    pub updated_at: String,
    pub title: String,
    pub url: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    #[serde(rename = "issues")]
    Issues,
    #[serde(rename = "pulls")]
    Pulls,
}

impl Display for IssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueType::Issues => write!(f, "issues"),
            IssueType::Pulls => write!(f, "pulls"),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CreateIssueOption {
    pub assignees: Option<Vec<String>>,
    pub body: Option<String>,
    pub closed: Option<bool>,
    pub due_date: Option<String>,
    pub labels: Option<Vec<i64>>,
    pub milestone: Option<i64>,
    pub r#ref: Option<String>,
    pub title: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GetIssuesOption {
    /// Whether issue is open or closed
    pub state: Option<State>,
    /// Comma separated list of labels. Fetch only issues that have any of this labels. Non existent labels are discarded
    pub labels: Option<Vec<String>>,
    /// Search string
    #[serde(rename = "q")]
    pub query: Option<String>,
    /// Filter by type (Issues or Pulls) if set
    #[serde(rename = "type")]
    pub issue_type: Option<IssueType>,
    /// Comma-separated list of milestone names or ids. It uses names and fall back to ids.
    /// Fetch only issues that have any of this milestones. Non existent milestones are discarded
    pub milestone: Option<String>,
    /// Only show items updated after the given time. This is a timestamp in RFC 3339 format
    pub since: Option<String>,
    /// Only show items updated before the given time. This is a timestamp in RFC 3339 format
    pub before: Option<String>,
    /// Only show items which were created by the given user
    pub created_by: Option<String>,
    /// Only show items for which the given user is assigned
    pub assigned_by: Option<String>,
    /// Only show items in which the given user was mentioned
    pub mentioned_by: Option<String>,
    /// Page number of results to return (1-based)
    pub page: Option<i64>,
    /// Page size of results
    pub limit: Option<i64>,
}

impl Client {
    /// Create an issue.
    /// If using deadline only the date will be taken into account, and time of day ignored.
    /// The only required field in the [CreateIssueOption] is `title`. All other fields are
    /// optional.
    /// This method will return the created issue.
    ///
    /// # Example
    /// ```rust
    /// # use teatime::{Client, CreateIssueOption, Auth};
    /// # async fn create_issue() {
    /// let client = Client::new("https://gitea.example.com",
    ///         Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let options = CreateIssueOption {
    ///     title: "my-new-issue".to_string(),
    ///     ..Default::default()
    /// };
    /// let issue = client.create_issue("owner", "repo", &options).await.unwrap();
    /// # }
    /// ```
    /// This will create a new issue with the title "my-new-issue" in the repository "owner/repo".
    pub async fn create_issue<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        options: &CreateIssueOption,
    ) -> Result<Issue> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let req = self
            .post(format!("repos/{owner}/{repo}/issues"))
            .json(options)
            .build()?;
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }

    /// Delete an issue.
    /// This will delete the issue with the given issue number.
    /// WARNING: This is irreversible and will not ask for confirmation. Use with caution.
    ///
    /// This method will return a 204 status code if the issue was successfully deleted.
    /// If the issue does not exist, this method will return a 404 status code.
    /// If the user is not authorized to delete the issue, this method will return a 403 status
    /// code.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use teatime::{Client, Auth};
    /// # async fn delete_issue() {
    /// let client = Client::new("https://gitea.example.com",
    ///         Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// client.delete_issue("owner", "repo", 1).await.unwrap();
    /// # }
    /// ```
    /// This will delete the issue #1 in the repository "owner/repo".
    pub async fn delete_issue<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        issue_number: i64,
    ) -> Result<()> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let req = self
            .delete(format!("repos/{owner}/{repo}/issues/{issue_number}"))
            .build()?;
        self.make_request(req).await?;
        Ok(())
    }

    /// List a repository's issues.
    /// The [GetIssuesOption] struct provides a number of optional fields to filter the results,
    /// but all fields are entirely optional.
    /// If you don't set any fields, you will get the most recent issues for the repository.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # use teatime::{Client, GetIssuesOption, Auth, State};
    /// # async fn get_issues() {
    /// let client = Client::new("https://gitea.example.com",
    ///         Auth::Token("e8ffd828994fc890156c56004e9f16eef224d8b0"));
    /// let options = GetIssuesOption {
    ///     state: Some(State::Open),
    ///     ..Default::default()
    /// };
    /// let issues = client.get_issues("owner", "repo", &options).await.unwrap();
    /// # }
    /// ```
    /// This will get all open issues in the repository "owner/repo".
    pub async fn get_issues<A: ToString, B: ToString>(
        &self,
        owner: A,
        repo: B,
        get_option: &GetIssuesOption,
    ) -> Result<Vec<Issue>> {
        let owner = owner.to_string();
        let repo = repo.to_string();
        let mut req = self.get(format!("repos/{owner}/{repo}/issues")).build()?;
        {
            let mut params = req.url_mut().query_pairs_mut();
            if let Some(state) = &get_option.state {
                params.append_pair("state", &state.to_string());
            }
            if let Some(labels) = &get_option.labels {
                params.append_pair("labels", &labels.join(","));
            }
            if let Some(query) = &get_option.query {
                params.append_pair("q", query);
            }
            if let Some(issue_type) = &get_option.issue_type {
                params.append_pair("type", &issue_type.to_string());
            }
            if let Some(milestone) = &get_option.milestone {
                params.append_pair("milestone", milestone);
            }
            if let Some(since) = &get_option.since {
                params.append_pair("since", since);
            }
            if let Some(before) = &get_option.before {
                params.append_pair("before", before);
            }
            if let Some(created_by) = &get_option.created_by {
                params.append_pair("created_by", created_by);
            }
            if let Some(assigned_by) = &get_option.assigned_by {
                params.append_pair("assigned_by", assigned_by);
            }
            if let Some(mentioned_by) = &get_option.mentioned_by {
                params.append_pair("mentioned_by", mentioned_by);
            }
            if let Some(page) = &get_option.page {
                params.append_pair("page", &page.to_string());
            }
            if let Some(limit) = &get_option.limit {
                params.append_pair("limit", &limit.to_string());
            }
        }
        let res = self.make_request(req).await?;
        self.parse_response(res).await
    }
}
