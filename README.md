# rust-project-generator

Generate a Rust-Project and a GitHub-Repository simultanieously.

## About

> Disclaimer:
> This is a WIP, it will further be improved.
> Code cleanup is on its way

Running the application will give you the ability to generate a new GitHub Repository and simultanieously a new cargo project in your current directory.
Eventually you'll be able to delete repos on GitHub and it'll delete the directory if found in your CWD.

## TODO

- [X] Get credentials, save in .rpg and authenticate to GH
- [X] Generate Repo using API
- [X] Clone repo and init cargo project inside
- [X] Add **Cargo.lock** to .gitignore 
    - Using GitHub .gitignore template and auto init for README
- [X] Initial commit and push of cargo project
- [ ] Remove repositories?
- [ ] Prettier 
- [ ] Refactor
