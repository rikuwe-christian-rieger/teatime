use build_it::Builder;
use serde::Serialize;

use crate::{
    error::Result,
    model::repos::{ObjectFormatName, Repository, TrustModel},
    Client,
};

/// Represents the options for creating a new repository.
/// The only required field is `name`.
#[derive(Debug, Clone, PartialEq, Serialize, Builder)]
#[build_it(into)]
#[serde(default)]
pub struct CreateRepoBuilder {
    /// Name of the organization to create the repository in.
    #[build_it(skip)]
    #[serde(skip)]
    org: String,
    /// Name of the repository to create.
    #[build_it(skip)]
    name: String,
    /// Whether the repository should be automatically initialized.
    /// This will create a README, LICENSE, and .gitignore file.
    auto_init: Option<bool>,
    /// Default branch of the repository.
    default_branch: Option<String>,
    /// Description of the repository.
    description: Option<String>,
    /// Optional Gitignore templates to use.
    /// Will be ignored if `auto_init` is false.
    gitignores: Option<String>,
    /// Optional Issue label-set to use.
    issue_labels: Option<String>,
    /// Optional LICENSE to use.
    license: Option<String>,
    /// Object Format Name of the underlying git repository.
    object_format_name: Option<ObjectFormatName>,
    /// Whether the repository is private.
    private: Option<bool>,
    /// Optional README template to use.
    /// Will be ignored if `auto_init` is false.
    readme: Option<String>,
    /// Whether the repository is a template.
    template: Option<bool>,
    /// Trust model for verifying commits in the repository.
    trust_model: Option<TrustModel>,
}

impl CreateRepoBuilder {
    pub fn new(org: impl ToString, name: impl ToString) -> Self {
        Self {
            org: org.to_string(),
            name: name.to_string(),
            auto_init: None,
            default_branch: None,
            description: None,
            gitignores: None,
            issue_labels: None,
            license: None,
            object_format_name: None,
            private: None,
            readme: None,
            template: None,
            trust_model: None,
        }
    }
    /// Send the request to create the repository.
    /// This will return the created [Repository].
    pub async fn send(&self, client: &Client) -> Result<Repository> {
        let org = &self.org;
        let req = client.post(format!("orgs/{org}/repos")).json(self).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
