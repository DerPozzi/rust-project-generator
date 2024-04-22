use std::fmt::Display;

#[derive(Debug)]
pub enum GitHubError {
    Authentication,
    RepoCreate,
    InitialCommit,
    AlreadyCreated
}

#[derive(Debug)]
pub enum CustomError {
    CargoErr(String),
    GitHubErr(GitHubError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::CargoErr(err) => write!(f, "ERROR: {}", err),
            CustomError::GitHubErr(GitHubError::Authentication) => {
                write!(f, "ERROR: Authentication failed")
            }
            CustomError::GitHubErr(GitHubError::RepoCreate) => {
                write!(f, "ERROR: Couldn't create GitHub repo")
            }
            CustomError::GitHubErr(GitHubError::InitialCommit) => {
                write!(f, "ERROR: Something went wrong during commit")
            }
            CustomError::GitHubErr(GitHubError::AlreadyCreated) => {
                write!(f, "ERROR: Repo already exists")
            }
        }
    }
}

impl std::error::Error for CustomError {}
