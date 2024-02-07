/*
    Function used to initialize the project
*/
use serde_json::json;
// use serde_json::Value;
use std::fs;
// use std::fs::OpenOptions;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use dialoguer::{theme::ColorfulTheme, Input};
use nodejs_semver::{Range, Version};
// model the project information
struct Project {
    name: String,
    version: String,
    description: String,
    entry_point: String,
    repo_url: String,
    author: String,
    license: String,
    private: bool,
}
impl Project {
    fn new_project(
        name: String,
        version: String,
        description: String,
        entry_point: String,
        repo_url: String,
        author: String,
        license: String,
        private: bool,
    ) -> Project {
        Project {
            name,
            version,
            description,
            entry_point,
            repo_url,
            author,
            license,
            private,
        }
    }
}
//handle project initialization

pub fn init_new_project(default:Option<String>) {
    // cache required variables
    let mut name = String::new();
    let mut version = String::new();
    let mut description = String::new();
    let mut entry_point = String::new();
    let mut repo_url = String::new();
    let mut author = String::new();
    let mut license = String::new();
    let mut private_input = String::new();
    let private = true;
    //get name of project
     name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Name")
         .default(default.unwrap())
        .interact_text()
        .unwrap();

    println!("Creating {}!", name);
    //get project version
    version = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Semver Version")
        .default("1.0.0".to_string())
        .interact_text()
        .unwrap();

    println!("Version: {}", version);
    //get project description
    description = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Description")
        .default("NodeJS Application".to_string())
        .interact_text()
        .unwrap();

    println!("Description: {}", description);
    // get projects entry point
    entry_point = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Default Entry Point")
        .default("index.js".to_string())
        .interact_text()
        .unwrap();

    println!("Entry Point: {}", entry_point);
    //get repo url
    repo_url = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Repository URL")
        // .default("1.0.0".to_string())
        .interact_text()
        .unwrap();

    println!("Repository: {}", repo_url);
    // get author
    author = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author")
        .default("".to_string())
        .interact_text()
        .unwrap();

    println!("Author: {}", author);
    // get license
    license = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Licence")
        .default("MIT".to_string().to_uppercase())
        .interact_text()
        .unwrap();

    println!("License: {}", license.to_uppercase());
    //  get is project private
    private_input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Private")
        .default("false".to_string())
        .interact_text()
        .unwrap();

    println!("Permissions: {}", private_input);
    //construct new project from user input
    let project = Project::new_project(
        name.trim().parse().unwrap(),
        version.trim().parse().unwrap(),
        description.trim().parse().unwrap(),
        entry_point.trim().to_lowercase().parse().unwrap(),
        repo_url.trim().parse().unwrap(),
        author.trim().parse().unwrap(),
        license.trim().to_uppercase().parse().unwrap(),
        private,
    );

    // now I can mess with the file system
    let dir_name = "./node_tests/".to_string();
    // first build a directory for the project
    //format project name
    fs::create_dir_all(dir_name).expect("failed to create directory");
    // create a package.json file with the project metadata
    create_package_json_file(project).unwrap();
    println!("success Saved package.json");
}
// create and save metadata to package.json
fn create_package_json_file(project: Project) -> io::Result<()> {
    let mut path_name = "./node_tests/package.json".to_string();
    //check if package file is available
    let file = fs::File::create(&mut path_name).expect("failed to create a package.json file");
    // use serde json create to create a json...
    //value from the Project Struct and write to a file
    let package_json_values = json!({
        "name": project.name,
        "version": project.version,
        "description":project.description,
        "main":project.entry_point,
        "repository":project.repo_url,
        "author":project.author,
        "license": project.license,
        "private":project.private
    });
    // write to package.json file
    let mut writer = BufWriter::new(file);
    // fs::write(&mut path_name, b"Lorem ipsum").expect("failed to write to package.json file");
    serde_json::to_writer_pretty(&mut writer, &package_json_values)?;
    writer.flush()?;
    Ok(())
}
