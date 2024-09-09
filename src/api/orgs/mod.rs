pub mod create;
pub mod create_repo;
pub mod delete;
pub mod edit;
pub mod get;
pub mod list_repos;
pub mod members;

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
    ///     .full_name("Organization")
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
    ///     .description("New description")
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

    /// List an [Organization](crate::model::orgs::Organization)'s
    /// [Repositories](crate::model::repos::Repository).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_repos() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let repos = client
    ///     .orgs("org-name")
    ///     .list_repos()
    ///     .page(2)
    ///     .limit(10)
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_repos(&self) -> list_repos::ListReposBuilder {
        list_repos::ListReposBuilder::new(self.name.clone())
    }

    /// Create a new [Repository](crate::model::repos::Repository) in an
    /// [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn create_repo() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .orgs("org-name")
    ///     .create_repo("repo-name")
    ///     .auto_init(true)
    ///     .license("mit")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn create_repo(&self, name: impl ToString) -> create_repo::CreateRepoBuilder {
        create_repo::CreateRepoBuilder::new(self.name.clone(), name)
    }

    /// List the members of an [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn list_members() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let members = client
    ///     .orgs("org-name")
    ///     .list_members()
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn list_members(&self) -> members::ListMembersBuilder {
        members::ListMembersBuilder::new(self.name.clone())
    }

    /// Check if a user is a member of an [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn is_member() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// let is_member = client
    ///     .orgs("org-name")
    ///     .is_member("username")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn is_member(&self, username: impl ToString) -> members::IsMemberBuilder {
        members::IsMemberBuilder::new(self.name.clone(), username)
    }

    /// Remove a user from an [Organization](crate::model::orgs::Organization).
    ///
    /// # Example
    /// ```
    /// # use gitea_sdk::{Client, Auth};
    /// # async fn remove_member() {
    /// let client = Client::new(
    ///     "https://gitea.example.com",
    ///     Auth::Token("your-token")
    /// );
    /// client
    ///     .orgs("org-name")
    ///     .remove_member("username")
    ///     .send(&client)
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn remove_member(&self, username: impl ToString) -> members::RemoveMemberBuilder {
        members::RemoveMemberBuilder::new(self.name.clone(), username)
    }
}
