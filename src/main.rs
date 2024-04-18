use crate::custom_error::{CustomError, GitHubError};
use colored::*;
use github_api::GitHubController;
use read_input::prelude::*;
use std::path::PathBuf;

mod custom_error;
mod github_api;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let mut config_file_path = homedir::get_my_home()
        .unwrap()
        .expect("ERROR: Couldn't receive home directory");

    config_file_path = PathBuf::from(config_file_path.to_string_lossy().to_string() + "/.rpg");

    let mut github_controller = github_api::GitHubController::new();

    println!(
        "Welcome to the {}ust {}roject {}enerator",
        "R".red(),
        "P".blue(),
        "G".green()
    );

    match config_at_startup(&config_file_path, &mut github_controller).await {
        Ok(()) => println!("Authentication successfull..."),
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    }

    // 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

    Ok(())
}

async fn config_at_startup(
    config_file_path: &PathBuf,
    controller: &mut GitHubController,
) -> Result<(), CustomError> {
    if !std::fs::metadata(&config_file_path).is_ok() {
        match setup_new_config(&config_file_path, controller).await {
            Err(err) => return Err(err),
            Ok(()) => {}
        }
    } else {
        let config_content = std::fs::read_to_string(&config_file_path).unwrap();
        let lines: Vec<&str> = config_content.lines().collect();
        let username = lines.get(0).unwrap().to_string();
        let personal_access_token = lines.get(1).unwrap().to_string();

        controller.set_username(username);
        controller.set_personal_access_token(personal_access_token);
    }

    match controller.test_github_access().await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}

async fn setup_new_config(
    config_path: &PathBuf,
    controller: &mut GitHubController,
) -> Result<(), crate::CustomError> {
    println!("Please enter your username and Personal Access Token to proceed.");
    print!("Username: ");
    let username = input::<String>().get();
    print!("Personal Access Token: ");
    let pat = input::<String>().get();
    std::fs::write(&config_path, format!("{}\n{}", username, pat)).unwrap();

    controller.set_username(username);
    controller.set_personal_access_token(pat);

    if let Err(err) = controller.test_github_access().await {
        return Err(CustomError::GitHubErr(GitHubError::Authentication));
    } else {
        println!("Setup complete.");
        return Ok(());
    }
}
