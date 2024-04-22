use reqwest::Client;
use serde::Serialize;

use crate::custom_error::{CustomError, GitHubError};

pub struct GitHubController {
    username: String,
    personal_access_token: String,
    url: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct GenerateRepoJson {
    name: String,
    description: String,
    private: bool,
    auto_init: bool,
    gitignore_template: String,
}

impl GitHubController {
    pub fn new() -> Self {
        GitHubController {
            username: String::new(),
            personal_access_token: String::new(),
            url: String::from("https://api.github.com/user/repos"),
            client: Client::new(),
        }
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }

    pub fn set_personal_access_token(&mut self, pat: String) {
        self.personal_access_token = pat
    }

    pub async fn generate_repository(
        &self,
        name: String,
        description: String,
        private: bool,
    ) -> Result<(), CustomError> {
        println!("Generating GitHub Repository...");
        let json = GenerateRepoJson {
            name,
            description,
            private,
            auto_init: true,
            gitignore_template: "Rust".to_string(),
        };

        let test = serde_json::json!(json);
        let request = self
            .client
            .post(&self.url)
            .header("User-Agent", &self.username)
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.personal_access_token)
            .json(&test)
            .send()
            .await;
        match request {
            Ok(resp) => {
                if resp.status() == 201 {
                    return Ok(());
                } else if resp.status() == 422 {
                    return Err(CustomError::GitHubErr(GitHubError::AlreadyCreated))
                } else {
                    return Err(CustomError::GitHubErr(GitHubError::RepoCreate));
                }
            }
            Err(err) => {
                println!("ERROR: {}", err);
                return Err(CustomError::GitHubErr(GitHubError::RepoCreate));
            }
        }
    }

    pub async fn test_github_access(&self) -> Result<(), crate::CustomError> {
        println!("Testing auth credentials...");
        let request = self
            .client
            .get(&self.url)
            .header("User-Agent", &self.username)
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.personal_access_token)
            .send();

        match request.await {
            Ok(res) => {
                let status = res.status();
                if status == 200 {
                    return Ok(());
                } else if status == 401 || status == 403 || status == 404 {
                    return Err(crate::custom_error::CustomError::GitHubErr(
                        GitHubError::Authentication,
                    ));
                } else {
                    panic!("Unexpected Status, aborting")
                }
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
