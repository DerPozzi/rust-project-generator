use crate::custom_error::CustomError;
use colored::*;
use github_api::GitHubController;
use read_input::prelude::*;
use std::path::PathBuf;

mod custom_error;
mod github_api;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!();
    let mut config_file_path = homedir::get_my_home()
        .unwrap()
        .expect("ERROR: Couldn't receive home directory");

    config_file_path = PathBuf::from(config_file_path.to_string_lossy().to_string() + "/.rpg");

    let mut github_controller = github_api::GitHubController::new();

    println!(
        "Welcome to the {}ust {}roject {}enerator:",
        "R".red(),
        "P".blue(),
        "G".green()
    );

    println!("");

    let (username, pat) = config_at_startup(&config_file_path);

    github_controller.set_username(username);
    github_controller.set_personal_access_token(pat);

    match github_controller.test_github_access().await {
        Ok(()) => println!("Authentication successfull..."),
        Err(err) => {
            println!("{}", err);
            print!("Enter new credentials [y/N]?");
            let choice = input::<char>().get();
            if choice == 'y' || choice == 'Y' {
                let (username, pat) = setup_new_config(&config_file_path);

                github_controller.set_username(username);
                github_controller.set_personal_access_token(pat);

                if let Err(err) = github_controller.test_github_access().await {
                    println!("{}", err);
                    return Err("Quitting application".to_string());
                }
            } else {
                return Err("Quitting application".to_string());
            }
        }
    }

    println!("Show main menu");

    show_menu();

    // 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

    Ok(())
}

fn show_menu() {
    println!();
    println!("===== $ {}{}{} $ =====", "R".red(), "P".blue(), "G".green());
    println!();
    println!("(1)\tSetup cargo project");
    println!("(2)\tChange GitHub credentials");
    println!();
    println!("(0)\tQuit")
}

fn config_at_startup(config_file_path: &PathBuf) -> (String, String) {
    let username: String;
    let personal_access_token: String;

    if !std::fs::metadata(&config_file_path).is_ok() {
        (username, personal_access_token) = setup_new_config(&config_file_path);
    } else {
        println!("Retrieving credentials from config...");
        let config_content = std::fs::read_to_string(&config_file_path).unwrap();
        let lines: Vec<&str> = config_content.lines().collect();
        username = lines.get(0).unwrap().to_string();
        personal_access_token = lines.get(1).unwrap().to_string();
    }
    (username, personal_access_token)
}

fn setup_new_config(config_path: &PathBuf) -> (String, String) {
    println!("Please enter your username and Personal Access Token to proceed.");
    print!("Username: ");
    let username = input::<String>().get();
    print!("Personal Access Token: ");
    let pat = input::<String>().get();
    let _ = std::fs::write(&config_path, format!("{}\n{}", username, pat));

    println!("Testing credentials...");

    (username, pat)
}
