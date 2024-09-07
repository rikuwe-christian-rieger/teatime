use build_it::Builder;
use serde::Serialize;

use crate::{error::Result, model::user::UserSettings, Client};

#[derive(Default, Debug)]
pub struct GetSettingsBuilder;

#[derive(Default, Debug, Builder, Serialize)]
#[build_it(into)]
pub struct UpdateSettingsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_view_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_repo_unit_hints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_activity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pronouns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
}

impl GetSettingsBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Sends the request to get the user's settings.
    pub async fn send(self, client: &Client) -> Result<UserSettings> {
        let req = client.get("user/settings").build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}

impl UpdateSettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sends the request to update the user's settings.
    pub async fn send(self, client: &Client) -> Result<UserSettings> {
        let req = client.patch("user/settings").json(&self).build()?;
        let res = client.make_request(req).await?;
        client.parse_response(res).await
    }
}
