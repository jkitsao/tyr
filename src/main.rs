use std::io;
//model project type via a struct
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
fn main() {
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
    let new_project = Project::new_project(
        name.trim().parse().unwrap(),
        version.trim().parse().unwrap(),
        description.trim().parse().unwrap(),
        entry_point.trim().parse().unwrap(),
        repo_url.trim().parse().unwrap(),
        author.trim().parse().unwrap(),
        license.trim().parse().unwrap(),
        private,
    );
    println!("Project information {:?}", new_project.name);
    println!("Project information {:?}", new_project.version);
    println!("Project information {:?}", new_project.description);
    println!("Project information {:?}", new_project.entry_point);
    println!("Project information {:?}", new_project.repo_url);
    println!("Project information {:?}", new_project.author);
    println!("Project information {:?}", new_project.license);
    println!("Project information {:?}", new_project.private);
}
