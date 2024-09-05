pub struct DeleteOrgBuilder {
    name: String,
}

impl DeleteOrgBuilder {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    /// Send the request to delete an [Organization](crate::model::orgs::Organization).
    pub async fn send(&self, client: &crate::Client) -> crate::error::Result<()> {
        let req = client.delete(&format!("orgs/{}", self.name)).build()?;
        client.make_request(req).await?;
        Ok(())
    }
}
