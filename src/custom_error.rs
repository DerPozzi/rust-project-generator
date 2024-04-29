use colored::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum GitHubError {
    Authentication,
    RepoCreate,
    InitialCommit,
    AlreadyCreated,
}

#[derive(Debug)]
pub enum CustomError {
    FilesystemErr(String),
    GitHubErr(GitHubError),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FilesystemErr(err) => write!(f, "{}: {}", "ERROR".red(), err),
            CustomError::GitHubErr(GitHubError::Authentication) => {
                write!(f, "{}: Authentication failed", "ERROR".red())
            }
            CustomError::GitHubErr(GitHubError::RepoCreate) => {
                write!(f, "{}: Couldn't create GitHub repo", "ERROR".red())
            }
            CustomError::GitHubErr(GitHubError::InitialCommit) => {
                write!(f, "{}: Something went wrong during commit", "ERROR".red())
            }
            CustomError::GitHubErr(GitHubError::AlreadyCreated) => {
                write!(f, "{}: Repo already exists", "ERROR".red())
            }
        }
    }
}

impl std::error::Error for CustomError {}
