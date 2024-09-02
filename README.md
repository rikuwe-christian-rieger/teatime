# teatime

NOTE: The crate name is now `gitea-sdk` because `teatime` was already taken on crates.io and I
forgot to check for that (oops).

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
// This will create a new repository with the name "my-new-repo" for the authenticated user.
let repo = client
    .user()
    .create_repo("my-new-repo")
    .send(&client)
    .await?;

let commits = client
    .repos("username", "awesome-repo")
    .get_commits()
    .limit(10)
    .send(&client)
    .await
    .unwrap();
```
