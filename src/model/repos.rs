use serde::{Deserialize, Serialize};

use crate::model::user::User;

/// Represents the format of the object in the repository.
/// Defaults to [ObjectFormatName::SHA1].
/// SHA1 is more widely supported, but SHA256 is more secure.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ObjectFormatName {
    #[default]
    #[serde(rename = "sha1")]
    SHA1,
    #[serde(rename = "sha256")]
    SHA256,
}

/// Represents the trust model for verifying commits in the repository.
/// Defaults to [TrustModel::Default] (obviously).
/// This determines when signatures are considered "trusted".
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrustModel {
    /// Use the default repository trust model for this installation.
    #[serde(rename = "default")]
    #[default]
    Default,
    /// Trust signatures signed by keys of collaborators.
    #[serde(rename = "collaborator")]
    Collaborator,
    /// Trust signatures that match the commiters (This matches GitHub and will force Gitea signed
    /// commits to have Gitea as the committer).
    #[serde(rename = "committer")]
    Committer,
    /// Trust signatures signed by keys of collaborators which match the committer.
    #[serde(rename = "collaboratorcommitter")]
    CollabroatorCommitter,
}

/// Represents a Gitea repository.
/// This struct is a subset of the full repository object.
/// Some fields the API provides (like external trackers) are not included here.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Repository {
    pub allow_fast_forward_only_merge: bool,
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_rebase_update: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    pub archived_at: String,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_allow_maintainer_edit: bool,
    pub default_branch: String,
    pub default_delete_branch_after_merge: bool,
    pub default_merge_style: String,
    pub description: String,
    pub empty: bool,
    pub fork: bool,
    pub forks_count: i64,
    pub full_name: String,
    pub has_actions: bool,
    pub has_issues: bool,
    pub has_packages: bool,
    pub has_projects: bool,
    pub has_pull_requests: bool,
    pub has_releases: bool,
    pub has_wiki: bool,
    pub html_url: String,
    pub id: i64,
    pub ignore_whitespace_conflicts: bool,
    pub internal: bool,
    pub language: String,
    pub languages_url: String,
    pub link: String,
    pub mirror: bool,
    pub mirror_interval: String,
    pub mirror_updated: String,
    pub name: String,
    pub object_format_name: ObjectFormatName,
    pub open_issues_count: i64,
    pub open_pr_counter: i64,
    pub original_url: String,
    pub owner: User,
    pub private: bool,
    pub release_counter: i64,
    pub size: i64,
    pub ssh_url: String,
    pub stars_count: i64,
    pub template: bool,
    pub updated_at: String,
    pub url: String,
    pub watchers_count: i64,
    pub website: String,
    pub wiki_branch: String,
}

/// Represents information about a user in the context of a commit.
///
/// NOTE: This is not the same as the [User] struct.
/// A CommitUser is not guaranteed to be a valid Gitea user.
/// A commit author can set the name and email tracked in this struct to anything they want.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommitUser {
    /// Date the commit was authored.
    pub date: String,
    /// Email of the user.
    pub email: String,
    /// Full name of the user.
    pub name: String,
}

/// Represents the actual commit object in the underlying git repository.
/// This struct is a subset of the full commit object.
/// It does not include the full commit tree or commit verification.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RepoCommit {
    /// Author of the commit (usually the person who originally wrote the code).
    pub author: CommitUser,
    /// The person who committed the code on behalf of the author. May be the same as the author.
    pub committer: CommitUser,
    /// The commit message.
    pub message: String,
    /// The API endpoint for the commit
    /// (https://gitea-host.com/api/v1/repos/{user}/{repo}/git/commits/{sha}.
    pub url: String,
}

/// Represents a commit in a repository.
/// This struct is a subset of the full commit object.
/// It does not include the affected files, parent commits or commit stats (additions and
/// deletions).
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Commit {
    /// The commit author's Gitea account.
    /// See [RepoCommit::author] for more information.
    /// NOTE: This is not guaranteed to be a valid Gitea user.
    /// Because of the nature of Git, this field can be null
    pub author: Option<User>,
    pub commit: RepoCommit,
    /// The committer's Gitea account.
    /// See [RepoCommit::committer] for more information.
    /// NOTE: This is not guaranteed to be a valid Gitea user.
    /// Because of the nature of Git, this field can be null
    pub committer: Option<User>,
    /// The URL to the commit on the Gitea instance.
    pub html_url: String,
    /// The SHA of the commit.
    pub sha: String,
    /// The API endpoint URL for the commit.
    pub url: String,
}
