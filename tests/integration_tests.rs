use std::env;

use gitea_sdk::{error::Result, Auth, Client};
use reqwest::Method;
use testcontainers::{
    core::{wait::HttpWaitStrategy, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};

static GITEA_USER: &str = "test-user";
static GITEA_PASSWORD: &str = "test-password";

static _ADMIN_USER: &str = "test-admin";
static _ADMIN_PASSWORD: &str = "test-password";

static GITEA_REPO: &str = "test-repo";
static GITEA_REPO_DESCRIPTION: &str = "a test repo";

#[tokio::test]
pub async fn test_client() {
    let wait_strategy = HttpWaitStrategy::new("/user/login")
        .with_port(3000.tcp())
        .with_method(Method::GET)
        .with_response_matcher(move |response| response.status().is_success());

    let container = GenericImage::new("teatime/test-image", "latest")
        .with_exposed_port(3000.tcp())
        .with_wait_for(WaitFor::Http(wait_strategy))
        .with_env_var("USER_UID", env::var("UID").unwrap_or("1000".to_string()))
        .with_env_var("USER_GID", env::var("GID").unwrap_or("1000".to_string()))
        .start()
        .await
        .expect("Failed to start Gitea container");

    let gitea_port = container
        .get_host_port_ipv4(3000)
        .await
        .expect("Failed to get Gitea port");
    let gitea_host = container
        .get_host()
        .await
        .expect("Failed to get Gitea host");

    let gitea_url = format!("http://{}:{}", gitea_host, gitea_port);
    let result = test(&gitea_url).await;

    // We always want to clean up the token, even if the tests fail. So we run this test outside of
    // the main test block.
    let delete = test_delete_token(&gitea_url, "gritty-token").await;

    container
        .stop()
        .await
        .expect("Failed to stop Gitea container");

    let mut panic = false;
    if let Err(e) = result {
        eprintln!("Failed to run tests: {}", e);
        panic = true;
    }
    if let Err(e) = delete {
        eprintln!("Failed to delete token: {}", e);
        panic = true;
    }
    if panic {
        panic!("Failed to run tests");
    }
}

pub async fn test(base_url: &str) -> Result<()> {
    println!("test_base_client");
    test_base_client(base_url).await?;

    println!("test_create_token");
    let token = test_create_token(base_url).await?;

    println!("test_get_user");
    test_get_user(base_url, &token).await?;

    println!("test_create_repo");
    test_create_repo(base_url, &token).await?;

    println!("test_user_list_repos");
    test_user_list_repos(base_url, &token).await?;

    println!("test_get_repo");
    test_get_repo(base_url, &token).await?;

    println!("test_edit_repo");
    test_edit_repo(base_url, &token).await?;

    // TODO: test forking - we need a second user for this
    // TODO: test migrating - we need a second repo for this

    println!("test_create_issue");
    test_create_issue(base_url, &token).await?;

    println!("test_list_issues");
    test_list_issues(base_url, &token).await?;

    println!("test_get_issue");
    test_get_issue(base_url, &token).await?;

    println!("test_search_issues");
    test_search_issues(base_url, &token).await?;

    println!("test_edit_issue");
    test_edit_issue(base_url, &token).await?;

    println!("test_delete_issue");
    test_delete_issue(base_url, &token).await?;

    println!("test_delete_repo");
    test_delete_repo(base_url, &token).await?;

    println!("test_create_private_repo");
    test_create_private_repo(base_url, &token).await?;

    println!("test_get_commits");
    test_get_commits(base_url, &token).await?;

    println!("test_search_repos");
    test_search_repos(base_url, &token).await?;

    println!("test_search_users");
    test_search_users(base_url, &token).await?;

    Ok(())
}

pub async fn test_base_client(base_url: &str) -> Result<Client> {
    Ok(Client::new(base_url, Auth::None::<String>))
}

pub async fn test_create_token(base_url: &str) -> Result<String> {
    let client = Client::new(base_url, Auth::Basic(GITEA_USER, GITEA_PASSWORD));
    let scopes = vec![
        "write:repository",
        "write:user",
        "write:issue",
        "write:organization",
    ];
    let token = client
        .user()
        .create_access_token(GITEA_USER, "gritty-token", scopes)
        .send(&client)
        .await?;
    Ok(token.sha1)
}

pub async fn test_delete_token(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Basic(GITEA_USER, GITEA_PASSWORD));
    client
        .user()
        .delete_access_token(GITEA_USER, token)
        .send(&client)
        .await?;
    Ok(())
}

pub async fn test_create_org(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let org = client
        .orgs("test-org")
        .create()
        .description("a test org".to_string())
        .send(&client)
        .await?;
    assert_eq!(org.name, "test-org");
    assert_eq!(org.description, Some("a test org".to_string()));
    Ok(())
}

pub async fn test_get_user(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let user = client.user().current().send(&client).await?;
    assert_eq!(user.login, GITEA_USER);
    Ok(())
}

pub async fn test_create_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client
        .user()
        .create_repo(GITEA_REPO)
        .description(GITEA_REPO_DESCRIPTION.to_string())
        .license("MIT".to_string())
        .auto_init(true)
        .send(&client)
        .await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_user_list_repos(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client
        .user()
        .list_repos()
        .limit(10)
        .page(1)
        .send(&client)
        .await?;
    assert_eq!(repo.len(), 1);
    assert_eq!(repo[0].owner.login, GITEA_USER);
    assert_eq!(repo[0].name, GITEA_REPO);
    assert_eq!(repo[0].description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_get_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client
        .repos(GITEA_USER, GITEA_REPO)
        .get()
        .send(&client)
        .await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_edit_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client
        .repos(GITEA_USER, GITEA_REPO)
        .edit()
        .send(&client)
        .await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_create_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issue = client
        .issues(GITEA_USER, GITEA_REPO)
        .create("test issue")
        .body("test issue body".to_string())
        .send(&client)
        .await?;
    assert_eq!(issue.title, "test issue");
    assert_eq!(issue.body, Some("test issue body".to_string()));
    Ok(())
}

pub async fn test_list_issues(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issues = client
        .issues(GITEA_USER, GITEA_REPO)
        .list()
        .send(&client)
        .await?;
    assert_eq!(issues.len(), 1);
    Ok(())
}

pub async fn test_get_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issue = client
        .issues(GITEA_USER, GITEA_REPO)
        .get(1)
        .send(&client)
        .await?;
    assert_eq!(issue.title, "test issue");
    Ok(())
}

pub async fn test_edit_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issue = client
        .issues(GITEA_USER, GITEA_REPO)
        .edit(1)
        .title("my new title".to_string())
        .unset_due_date(true)
        .send(&client)
        .await?;
    assert_eq!(issue.title, "my new title");
    Ok(())
}

pub async fn test_search_issues(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issues = client.search().issues().send(&client).await?;
    assert_eq!(issues.len(), 1);
    Ok(())
}

pub async fn test_delete_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    client
        .issues(GITEA_USER, GITEA_REPO)
        .delete(1)
        .send(&client)
        .await?;
    Ok(())
}

pub async fn test_delete_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    client
        .repos(GITEA_USER, GITEA_REPO)
        .delete()
        .send(&client)
        .await?;
    Ok(())
}

pub async fn test_create_private_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client
        .user()
        .create_repo(GITEA_REPO)
        .license("MIT".to_string())
        .description(GITEA_REPO_DESCRIPTION.to_string())
        .auto_init(true)
        .private(true)
        .send(&client)
        .await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    assert!(repo.private);
    Ok(())
}

pub async fn test_get_commits(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let commits = client
        .repos(GITEA_USER, GITEA_REPO)
        .get_commits()
        .send(&client)
        .await?;
    assert_eq!(commits.len(), 1);
    Ok(())
}

pub async fn test_search_repos(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repos = client.search().repos().send(&client).await?;
    assert_eq!(repos.len(), 1);
    Ok(())
}

pub async fn test_search_users(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let users = client
        .search()
        .users()
        .query(GITEA_USER.to_string())
        .send(&client)
        .await?;
    assert_eq!(users.len(), 1);
    Ok(())
}
