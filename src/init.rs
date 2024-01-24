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

pub fn init_new_project() {
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
    println!("Enter the name of your project ");
    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a valid project name");
    // println!("create a project called {}", name);
    //get project version
    println!("Version");
    io::stdin()
        .read_line(&mut version)
        .expect("Please enter a valid version");
    //get project description
    println!("Description");
    io::stdin()
        .read_line(&mut description)
        .expect("Please enter a valid description");
    // get projects entry point
    println!("Entry Point");
    io::stdin()
        .read_line(&mut entry_point)
        .expect("Please enter a valid entry point");
    //get repo url
    println!("Repository url");
    io::stdin()
        .read_line(&mut repo_url)
        .expect("Please enter a valid git url");
    // get author
    println!("Author");
    io::stdin()
        .read_line(&mut author)
        .expect("Please enter a valid author name");
    // get license
    println!("Licence");
    io::stdin()
        .read_line(&mut license)
        .expect("Please enter a valid License type");
    //  get is project private
    println!("Private");
    io::stdin()
        .read_line(&mut private_input)
        .expect("Invalid input");
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

    // now i can mess with the file system
    // first build a directory for the project
    //format project name
    let dir_name = format!("./node_tests/");
    fs::create_dir_all(dir_name).expect("failed to create directory");
    // create a package.json file with the project metadata
    create_package_json_file(project).unwrap();
    println!("success Saved package.json");
}
// create and save metadata to package.json
fn create_package_json_file(project: Project) -> io::Result<()> {
    let mut path_name = format!("./node_tests/package.json");
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
