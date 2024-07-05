# teatime

Teatime is a simple Gitea API client for Rust. It's goal is to give you the ability to write
exactly as much code as you need to interact with the specific parts of the Gitea API you need,
but no more.

# Usage

The main way to interact with the Gitea API is through the `Client` struct. You can create a
new `Client` by calling `Client::new` with the base URL of your Gitea instance and a personal
token. Teatime does currently not support basic HTML or OAuth2 authentication.

Once you have obtained a `Client`, you can interact with the Gitea API by calling the various
methods the instance provides. This example will create a new Repository and get the 10 last
commits of the repository `username/awesome-repo`:
```rust
let client = Client::new("https://gitea.example.com".to_string(), "your-token".to_string());
let create_option = CreateRepoOption {
    // `name` is the only required field
    name: "my-new-repo".to_string(),
    ..Default::default()
};
// This will create a new repository with the name "my-new-repo" for the authenticated user.
let repo = client.user_create_repository(&create_option).await.unwrap();

let get_option = GetCommitsOption {
    // `GetCommitsOption` has a number of optional fields to filter the results,
    // but none are required. In this example, we're just setting the `limit` to 10 to
    // only get the 10 most recent commits.
    limit: Some(10),
    ..Default::default()
};
let commits = client.get_commits("username", "awesome-repo",  get_option).await.unwrap();
```
