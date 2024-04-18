use crate::custom_error::GitHubError;

pub struct GitHubController {
    username: String,
    personal_access_token: String,
    url: String,
}

impl GitHubController {
    pub fn new() -> Self {
        GitHubController {
            username: String::new(),
            personal_access_token: String::new(),
            url: String::from("https://api.github.com/user/repos"),
        }
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username
    }

    pub fn set_personal_access_token(&mut self, pat: String) {
        self.personal_access_token = pat
    }

    pub async fn test_github_access(&self) -> Result<(), crate::CustomError> {
        let client = reqwest::Client::new();
        let request = client
            .get(&self.url)
            .header("User-Agent", &self.username)
            .header(
                "Authorization",
                format!("Bearer {}", self.personal_access_token),
            )
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
