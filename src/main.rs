// use clap::builder::Str;
use clap::Error;
mod http;
mod semvar;
mod unzip;
use serde_json::json;
use std::collections::BTreeMap;
// use std::fmt::{self};
// use std::collections::HashMap;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
// use std::io::copy;
// use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
// use std::path;

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
    //TODO: Get cli args first and add to vector
    let args: Vec<String> = env::args().collect();
    let _def_arg = String::from(&args[1]);
    let primary_arg = &args[2];
    //secondary args depends on primary (used for dependancies)
    let secondary_arg = args.get(3);

    // dbg!(def_arg, primary_arg);
    // dbg!(args);
    //first match the primary for either init or add
    match primary_arg.as_ref() {
        "init" => {
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
        //handle the add command
        "add" => {
            let dep = secondary_arg.expect("Provide a valid dependancy");
            resolve_package_from_registry(dep.to_string());
            //read the package.json of installed dep to get the next one
            //
            resolve_next_dep(dep.to_string());
            // println!("{:?}", dep);
        }
        // install all dep in current package.json file
        "install" => println!("proceed to installing dependencies"),
        "tyr" => println!("proceed to installing dependencies"),
        _ => println!("nothing special"),
    };
}
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
//resolve dependancy/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided but i'll start
fn resolve_package_from_registry(dep: String) {
    let (name, version) = semvar::split_package_version(&dep);
    // println!("package is {}, and the version is {}", name, version);
    match version.as_ref() {
        "latest" => {
            println!("i'll dowload the latest version of {}", name);
            // call package installer
            let install_db = package_installer(name, version);
            //create a lock file and update package.json dependancies
            generate_lock_file(install_db).unwrap(); //also updates/creates dep in package.json
        }
        //semvar string has been passed
        _ => {
            // version number has been passed
            // let semvar_version = ;
            // let _semvar_version = resolve_full_version(Version::parse(&version).unwrap());
            // call package installer with semvar version
            let install_db = package_installer(name, version);
            generate_lock_file(install_db).unwrap(); //also updates/creates dep in package.json
                                                     //semver as version
        }
    }
}

// install package to disk
//installer function that resolves remote packages and arranges to disk
fn package_installer(name: String, version: String) -> HashMap<String, Value> {
    // call download package from registry function with
    let resolved = http::resolve_remote_package(name, version).unwrap();
    // Now 'resolved' contains the parsed JSON data as a HashMap
    let dist = resolved.get("dist").unwrap();
    let version = resolved.get("version").unwrap();
    let tarball = dist.get("tarball").unwrap();
    let _integrity = dist.get("integrity").unwrap();
    let name = resolved.get("name").unwrap();
    // println!("the tar is {:?} and version is {:?}", tarball, version);
    println!("proceeding to install {}  version {}", name, version);
    unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
    resolved
}
//Generate lock files with package name,version,resolve url, and integrity checksum
fn generate_lock_file(package: HashMap<String, Value>) -> Result<(), Error> {
    //model lock content
    struct LockFile {
        name: String,
        version: String,
        resolved: String,
        integrity: String,
    }
    //formatter function returns placeholders without double quotes around the name, version, resolved, and integrity
    impl LockFile {
        fn format_for_lock_file(&self) -> String {
            format!(
                "\n \n {}@{}:\n version {}\n  resolved {}\n  integrity {}",
                self.name, self.version, self.version, self.resolved, self.integrity
            )
        }
    }
    //get required values from hashmap
    let dist = package.get("dist").unwrap();
    let version = package.get("version").unwrap();
    let tarball = dist.get("tarball").unwrap();
    let integrity = dist.get("integrity").unwrap();
    let name = package.get("name").unwrap();
    //create a lock file with fs package and write to it
    let mut path_name = format!("./node_tests/tyr.lock");
    // fs::File::create(path)
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mut path_name)
        .expect("failed to create a package.lock file");
    //construct a new lock object with package metadata
    let lock = LockFile {
        name: name.to_string(),
        version: version.to_string(),
        integrity: integrity.to_string(),
        resolved: tarball.to_string(),
    };
    // let formatted = lock.format_for_lock_file();
    write!(file, "{}", &lock.format_for_lock_file())?;
    println!("Saved lockfile");
    update_package_jason_dep(package).unwrap();

    Ok(())
}
//function to update dependancy packages after installation
//first model the dep struct and package struct

fn update_package_jason_dep(package: HashMap<String, Value>) -> io::Result<()> {
    //read contents of the file
    let path_name = format!("./node_tests/package.json");
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    // Read the JSON contents of the file and assign to Hashmap.
    let json_file_data: BTreeMap<String, Value> = serde_json::from_reader(reader)?;

    let version: String = package
        .get("version")
        .unwrap()
        .to_string()
        .trim_matches('"')
        .parse()
        .unwrap();
    let name: String = package
        .get("name")
        .unwrap()
        .to_string()
        .trim_matches('"')
        .parse()
        .unwrap();
    let is_dep_init = json_file_data.contains_key("dependencies");

    match is_dep_init {
        true => {
            // println!("Dep object detected we should append to json");
            //update the dep object with installed package metadata
            update_dep_obj(json_file_data, name.clone(), version).unwrap();
            resolve_next_dep(name.to_string());
            // println!("maybe read {} package and see", name);
        }
        false => {
            println!("Dep object not found we should create then add");
            // probably the first package
            create_dep_obj(json_file_data, name, version).unwrap();
        }
    }
    // println!("is dependencies initiated in project {:?}", is_dep_init);
    Ok(())
}
// create dep object on the package.json file with new package metadata
fn create_dep_obj(
    mut metadata: BTreeMap<String, Value>,
    name: String,
    version: String,
) -> io::Result<()> {
    // create the json value with serde
    let value = json!({
        "dependencies": {
            name:version
        }
    });
    // metadata.insert(k, v)
    let dep_value: BTreeMap<String, Value> = serde_json::from_value(value).unwrap();
    //merge the 2 data structures
    metadata.extend(dep_value);
    let result = json!(metadata);
    let mut path_name = format!("./node_tests/package.json");
    let file = fs::File::create(&mut path_name).expect("failed to create a package.json file");
    // write to package.json file
    let mut writer = BufWriter::new(file);
    // fs::write(&mut path_name, b"Lorem ipsum").expect("failed to write to package.json file");
    serde_json::to_writer_pretty(&mut writer, &result)?;
    writer.flush()?;
    Ok(())
}
//update the dependency object
fn update_dep_obj(
    mut metadata: BTreeMap<String, Value>,
    name: String,
    version: String,
) -> io::Result<()> {
    // create the json value with serde
    let current_dep: Value = metadata.get_mut("dependencies").unwrap().clone();
    //append installed package meta on the current_dep value
    let mut temp_json: HashMap<String, String> = serde_json::from_value(current_dep).unwrap();
    temp_json.insert(name, version);
    //update package.json instance with new dependancies
    if let Some(x) = metadata.get_mut("dependencies") {
        *x = json!(temp_json);
    };
    // println!("metadata {:?}", metadata);
    //write output to file
    //serialize first
    let results = json!(metadata);
    let mut path_name = format!("./node_tests/package.json");
    let file = fs::File::create(&mut path_name).expect("failed to create a package.json file");
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &results)?;
    Ok(())
}
//fetch next dep after instalation of package
//to resolve the next dependancies
fn resolve_next_dep(name: String) {
    let path_name = format!("./node_tests/node_modules/{}/package.json", name);
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    // Read the JSON contents of the file and assign to Hashmap.
    let mut json_file_data: BTreeMap<String, Value> = serde_json::from_reader(reader).unwrap();
    //match to check id dep is available
    match json_file_data.contains_key("dependencies") {
        true => {
            let next_dep: Value = json_file_data.get_mut("dependencies").unwrap().clone();
            let temp_json: HashMap<String, String> = serde_json::from_value(next_dep).unwrap();
            let it = temp_json.iter();
            // let string_name=String
            // println!("**** The next package is {:?}", temp_json);
            //parse the values before calling install
            //check if theres dep
            match temp_json.is_empty() {
                true => {
                    print!("********  no more dependencies *********");
                }
                false => {
                    // println!("{:?}", it);
                    // Iterate over the keys and values of the hashmap
                    for (key, value) in it {
                        // Remove backticks from the value
                        let new_value = value.replace('^', "");
                        println!("Key: {}, Value: {}", key, value);
                        let package_name = format!("{}@{}", key, new_value);
                        // println!("{}", package_name);
                        //
                        println!("****** installing the next one {:?} \n \n", package_name);
                        resolve_package_from_registry(package_name.to_string());
                    }
                    //format the map values to stringproceeding to install
                }
            }
        }
        //package.json does not contain dependency field
        false => {
            println!("No dependencies in package")
        }
    }

    // check if dep has been installed
}
