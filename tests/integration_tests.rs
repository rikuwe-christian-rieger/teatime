use std::{env, fs};

use reqwest::Method;
use teatime::{
    error::Result, Auth, Client, CreateAccessTokenOption, CreateForkOption, CreateIssueOption,
    CreateRepoOption, GetCommitsOption, GetIssuesOption, ListForksOption, SearchRepositoriesOption,
};
use testcontainers::{
    core::{wait::HttpWaitStrategy, IntoContainerPort, Mount, WaitFor},
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
pub async fn test_teatime() {
    let wait_strategy = HttpWaitStrategy::new("/user/login")
        .with_port(3000.tcp())
        .with_method(Method::GET)
        .with_response_matcher(move |response| response.status().is_success());

    let data = Mount::bind_mount(
        format!(
            "{}/gitea",
            std::env::current_dir().unwrap().to_str().unwrap()
        ),
        "/data",
    );

    fs::create_dir_all("gitea/gitea/conf").expect("Failed to create gitea directory");
    fs::copy("test-data/gitea.db", "gitea/gitea/gitea.db").expect("Failed to copy gitea.db");
    fs::copy("test-data/app.ini", "gitea/gitea/conf/app.ini").expect("Failed to copy app.ini");

    let container = GenericImage::new("gitea/gitea", "latest")
        .with_exposed_port(3000.tcp())
        .with_wait_for(WaitFor::Http(wait_strategy))
        .with_mount(data)
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

    let remove_gitea = fs::remove_dir_all("gitea/gitea");
    let remove_git = fs::remove_dir_all("gitea/git");

    let mut panic = false;
    if let Err(e) = result {
        eprintln!("Failed to run tests: {}", e);
        panic = true;
    }
    if let Err(e) = delete {
        eprintln!("Failed to delete token: {}", e);
        panic = true;
    }
    if let Err(e) = remove_git {
        eprintln!("Failed to remove gitea/git directory: {}", e);
        panic = true;
    }
    if let Err(e) = remove_gitea {
        eprintln!("Failed to remove gitea/gitea directory: {}", e);
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

    println!("test_get_repo");
    test_get_repo(base_url, &token).await?;

    // TODO: test forking - we need a second user for this

    println!("test_create_issue");
    test_create_issue(base_url, &token).await?;

    println!("test_get_issues");
    test_get_issues(base_url, &token).await?;

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
    Ok(())
}

pub async fn test_base_client(base_url: &str) -> Result<Client> {
    Ok(Client::new(base_url, Auth::None::<String>))
}

pub async fn test_create_token(base_url: &str) -> Result<String> {
    let client = Client::new(base_url, Auth::Basic(GITEA_USER, GITEA_PASSWORD));
    let options = CreateAccessTokenOption {
        name: "gritty-token".to_string(),
        scopes: Some(vec![
            "write:repository".into(),
            "write:user".into(),
            "write:issue".into(),
            "write:organization".into(),
        ]),
    };
    let token = client.create_access_token(GITEA_USER, &options).await?;
    Ok(token.sha1)
}

pub async fn test_delete_token(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Basic(GITEA_USER, GITEA_PASSWORD));
    client.delete_access_token(GITEA_USER, token).await?;
    Ok(())
}

pub async fn test_get_user(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let user = client.get_authenticated_user().await?;
    assert_eq!(user.login, GITEA_USER);
    Ok(())
}

pub async fn test_create_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let options = CreateRepoOption {
        name: GITEA_REPO.to_string(),
        license: "MIT".to_string(),
        description: GITEA_REPO_DESCRIPTION.to_string(),
        auto_init: true,
        ..Default::default()
    };
    let repo = client.user_create_repository(&options).await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_get_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let repo = client.get_repository(GITEA_USER, GITEA_REPO).await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    Ok(())
}

pub async fn test_create_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let options = CreateIssueOption {
        title: "test issue".to_string(),
        body: Some("test issue body".to_string()),
        ..Default::default()
    };
    let issue = client
        .create_issue(GITEA_USER, GITEA_REPO, &options)
        .await?;
    assert_eq!(issue.title, "test issue");
    assert_eq!(issue.body, Some("test issue body".to_string()));
    Ok(())
}

pub async fn test_get_issues(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let issues = client
        .get_issues(GITEA_USER, GITEA_REPO, &GetIssuesOption::default())
        .await?;
    assert_eq!(issues.len(), 1);
    Ok(())
}

pub async fn test_delete_issue(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    client.delete_issue(GITEA_USER, GITEA_REPO, 1).await?;
    Ok(())
}

pub async fn test_delete_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    client.delete_repository(GITEA_USER, GITEA_REPO).await?;
    Ok(())
}

pub async fn test_create_private_repo(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let options = CreateRepoOption {
        name: GITEA_REPO.to_string(),
        license: "MIT".to_string(),
        description: GITEA_REPO_DESCRIPTION.to_string(),
        auto_init: true,
        private: true,
        ..Default::default()
    };
    let repo = client.user_create_repository(&options).await?;
    assert_eq!(repo.owner.login, GITEA_USER);
    assert_eq!(repo.name, GITEA_REPO);
    assert_eq!(repo.description, GITEA_REPO_DESCRIPTION);
    assert!(repo.private);
    Ok(())
}

pub async fn test_get_commits(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let options = GetCommitsOption::default();
    let commits = client.get_commits(GITEA_USER, GITEA_REPO, &options).await?;
    assert_eq!(commits.len(), 1);
    Ok(())
}

pub async fn test_search_repos(base_url: &str, token: &str) -> Result<()> {
    let client = Client::new(base_url, Auth::Token(token));
    let options = SearchRepositoriesOption::default();
    let repos = client.search_repositories(&options).await?;
    assert_eq!(repos.len(), 1);
    Ok(())
}
