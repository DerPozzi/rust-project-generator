use crate::custom_error::CustomError;
use colored::*;
use github_api::GitHubController;
use read_input::prelude::*;
use std::{path::PathBuf, process::Command, thread::sleep, time::Duration};

mod custom_error;
mod github_api;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!();
    let mut config_file_path = homedir::get_my_home()
        .unwrap()
        .expect("ERROR: Couldn't receive home directory");

    config_file_path.push(".rpg");

    let mut github_controller = github_api::GitHubController::new();

    println!(
        "Welcome to the {}ust {}roject {}enerator:",
        "R".red(),
        "P".blue(),
        "G".green()
    );

    println!();

    let (username, personal_access_token) = load_config_at_start(&config_file_path);

    github_controller.set_username(username);
    github_controller.set_personal_access_token(personal_access_token);

    if let Err(err) = github_controller.test_github_access().await {
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
    } else {
        println!("Authentication successfull...")
    }

    println!("Starting {}{}{}...", "R".red(), "P".blue(), "G".green());
    sleep(Duration::from_secs_f64(1.5));

    loop {
        clear_screen();
        show_menu();

        print!("{}{}{} ", ">".red(), ">".blue(), ">".green());
        match input::<u8>().get() {
            0 => {
                println!();
                println!("Quitting program...");
                break;
            }
            1 => {
                println!();
                if let Err(err) = generate_new_project(&github_controller).await {
                    println!("{err}")
                }
            }
            2 => {
                if let Err(err) =
                    change_credentials(&config_file_path, &mut github_controller).await
                {
                    return Err(format!("{}", err));
                } else {
                    println!("Change successfull, return to menu...");
                }
            }
            _ => {}
        }
        sleep(Duration::from_secs_f64(1.5))
    }

    Ok(())
}

async fn generate_new_project(github_controller: &GitHubController) -> Result<(), CustomError> {
    print!("Enter the project name [e.g. rust-project-generator]: ");
    let project_name = input::<String>().get();
    print!("Private [y/N]? ");
    let private = match input::<char>().get() {
        'y' => true,
        'Y' => true,
        _ => false,
    };
    println!("Short description (optional): ");
    let description = input::<String>().get();
    if let Err(err) = github_controller
        .generate_repository(project_name.clone(), description, private)
        .await
    {
        return Err(err);
    } else {
        println!("Successfully generated repository");
        println!("Cloning repository...");

        Command::new("git")
            .arg("clone")
            .arg(format!(
                "git@github.com:{}/{}.git",
                github_controller.get_username(),
                project_name
            ))
            .spawn()
            .expect("Couldn't spawn task")
            .wait()
            .expect("Couldn't run git command");
    }
    let current_dir = std::env::current_dir().expect("Couldn't get current directory...");
    println!(
        "Generating project {} in {}",
        project_name,
        current_dir.as_os_str().to_str().unwrap()
    );

    let mut target_path = current_dir.clone();
    target_path.push(&project_name);

    if std::env::set_current_dir(target_path).is_err() {
        return Err(CustomError::FilesystemErr("Couldn't open directory".to_string()));
    }

    if let Err(err) = Command::new("cargo").arg("init").spawn().unwrap().wait() {
        return Err(CustomError::FilesystemErr(err.to_string()));
    } else {
        println!("Successfully generated project.")
    }

    inital_commit()?;

    std::env::set_current_dir(current_dir).unwrap();
    sleep(Duration::from_secs(3));
    Ok(())
}

fn inital_commit() -> Result<(), CustomError> {
    println!("Pushing initial commit to origin...");
    if let Err(_) = Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("Couldn't start git command")
        .wait()
    {
        return Err(CustomError::GitHubErr(
            custom_error::GitHubError::InitialCommit,
        ));
    }
    if let Err(_) = Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg("Initial commit")
        .spawn()
        .expect("Couldn't start git command")
        .wait()
    {
        return Err(CustomError::GitHubErr(
            custom_error::GitHubError::InitialCommit,
        ));
    }
    Command::new("git")
        .arg("push")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Ok(())
}

async fn change_credentials(
    config_file_path: &PathBuf,
    github_controller: &mut GitHubController,
) -> Result<(), CustomError> {
    println!();
    let (username, pat) = setup_new_config(config_file_path);
    github_controller.set_username(username);
    github_controller.set_personal_access_token(pat);

    if let Err(err) = github_controller.test_github_access().await {
        Err(err)
    } else {
        Ok(())
    }
}

fn show_menu() {
    println!();
    println!("===== $ {}{}{} $ =====", "R".red(), "P".blue(), "G".green());
    println!();
    println!("(1)\tSetup cargo project");
    println!("(2)\tChange GitHub credentials");
    println!();
    println!("(0)\tQuit");
    println!()
}

fn clear_screen() {
    Command::new("clear")
        .spawn()
        .expect("Couldn't spawn 'clear' thread")
        .wait()
        .unwrap();
}

fn load_config_at_start(config_file_path: &PathBuf) -> (String, String) {
    let username: String;
    let personal_access_token: String;

    if std::fs::metadata(config_file_path).is_err() {
        (username, personal_access_token) = setup_new_config(config_file_path);
    } else {
        println!("Retrieving credentials from config...");
        let config_content = std::fs::read_to_string(config_file_path).unwrap();
        let lines: Vec<&str> = config_content.lines().collect();
        username = lines.first().unwrap().to_string();
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
    let _ = std::fs::write(config_path, format!("{}\n{}", username, pat));

    println!("Testing credentials...");

    (username, pat)
}
