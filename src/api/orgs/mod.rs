pub mod create;
pub mod delete;
pub mod edit;
pub mod get;

pub struct Orgs {
    pub name: String,
}

impl Orgs {
    /// Create a new [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_org() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .orgs("org-name")
    ///     .create()
    ///     .full_name("Organization".to_string())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will create a new organization with the name "org-name" and the full name
    /// "Organization
    pub fn create(&self) -> create::CreateOrgBuilder {
        create::CreateOrgBuilder::new(self.name.clone())
    }

    /// Get an [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn get_org() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let org = client
    ///     .orgs("org-name")
    ///     .get()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will get the organization with the name "org-name".
    pub fn get(&self) -> get::GetOrgBuilder {
        get::GetOrgBuilder::new(self.name.clone())
    }

    /// Delete an [Organization](crate::model::orgs::Organization).
    /// This will delete the organization with the name "org-name".
    /// This action is irreversible.
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn delete_org() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .orgs("org-name")
    ///     .delete()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will delete the organization with the name "org-name".
    pub fn delete(&self) -> delete::DeleteOrgBuilder {
        delete::DeleteOrgBuilder::new(self.name.clone())
    }

    /// Edit an [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn edit_org() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .orgs("org-name")
    ///     .edit()
    ///     .description("New description".to_string())
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// This will edit the organization with the name "org-name" to have the description "New
    /// description".
    pub fn edit(&self) -> edit::EditOrgBuilder {
        edit::EditOrgBuilder::new(self.name.clone())
    }
}
