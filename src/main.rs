use clap::builder::Str;
use clap::Error;
// use clap::builder::Str;
// use clap::Error;
// use clap::builder::Str;
// use reqwest;
// use reqwest::header::ALLOW;
// use reqwest::Client;
use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};
use serde_json::json;
use std::collections::BTreeMap;
// use std::fmt::{self};
// use std::collections::HashMap;
use flate2::read::GzDecoder; // Add this import for Gzip support
use serde_json::{from_slice, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::copy;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
// use std::ptr::metadata;
// use std::string::ParseError;
use tar::Archive;
use ureq::{Agent, AgentBuilder};
// registry url
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
// supported commands
enum Commands {
    Init(String),
    Add(String),
}
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
    let (name, version) = resolve_semver(dep);
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
            let semvar_version = resolve_full_version(Version::parse(&version).unwrap());
            // let map: HashMap<String, Value> = serde_json::from_value(&semvar_version).unwrap();
            println!("package you want is {:?} semvar {:?}", name, semvar_version);
            // resolve_remote_package(name, "latest".to_string()).unwrap();
            // call package installer with semvar version
            let install_db = package_installer(name, version);
            generate_lock_file(install_db).unwrap(); //also updates/creates dep in package.json
                                                     //semver as version
        }
    }
}
//resolve semver versions from the name
fn resolve_semver(name: String) -> (String, String) {
    let package;
    let version;
    if name.contains("@") {
        let v: Vec<&str> = name.split('@').collect();
        package = v[0];
        version = v[1];
        (package.to_string(), version.to_string())
    } else {
        (name.to_string(), "latest".to_string())
    }
}
//function to send http request
fn resolve_remote_package(
    name: String,
    version: String,
) -> Result<HashMap<String, Value>, ureq::Error> {
    let url = format!("{}/{}/{}", NPM_REGISTRY_URL, name, version);
    let body: String = ureq::get(&url)
        .set(
            "ALLOW",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        )
        .call()?
        .into_string()?;
    // println!("body {:#?}", body);
    let map: HashMap<String, Value> = serde_json::from_str(&body).unwrap();
    Ok(map)
}
/// If a version comparator has the major, patch and minor available a string version will be returned with the resolved version.
/// This version string can be used to retrieve a package version from the NPM registry.
/// If the version is not resolvable without requesting the full package data, None will be returned.
/// None will also be returned if the version operator is Op::Less (<?.?.?) because we need all versions to get the latest version less than this
pub fn resolve_full_version(semantic_version: Version) -> String {
    // println!("major {}", semantic_version.major)
    stringify_from_numbers(
        semantic_version.major,
        semantic_version.minor,
        semantic_version.patch,
    )
}
fn stringify_from_numbers(major: u64, minor: u64, patch: u64) -> String {
    format!("{}.{}.{}", major, minor, patch)
}
// install package to disk
fn extract_tarball_to_disk(url: &str, package_name: &str) {
    // URL of the tar file
    // let url = "https://example.com/path/to/your.tar.gz";

    // Destination folder
    // let dest_folder = "./node_tests/node_modules";
    let dest_folder = format!("./node_tests/node_modules/{}", package_name);

    // Create the destination folder if it doesn't exist
    if !Path::new(dest_folder.as_str()).exists() {
        std::fs::create_dir_all(&dest_folder).expect("Failed to create destination folder");
    }

    // Download the tar file using ureq
    let response = ureq::get(url).call().expect("failed to download tar");
    // Create a temporary file to store the downloaded tar file
    let mut temp_file = fs::File::create("temp.tar.gz").expect("Failed to create temp file");

    // Copy the response body to the temporary file
    copy(&mut response.into_reader(), &mut temp_file)
        .expect("Failed to copy response body to file");

    // Open the downloaded tar file
    let tar_file = fs::File::open("temp.tar.gz").expect("Failed to open tar file");
    // Use Gzip decoder for decompression
    let tar_reader = BufReader::new(GzDecoder::new(tar_file));
    // Create a tar archive from the file
    let mut archive = Archive::new(tar_reader);
    // Extract the contents of the tar file to the custom project folder
    // ** we also remove the default /package from the tar returned by NPM**
    archive
        .entries()
        .expect("Failed to get tar entries")
        .for_each(|entry| {
            let mut entry = entry.expect("Failed to get tar entry");

            // Handle variations in the directory structure
            let entry_path = entry.path().expect("Failed to get entry path");
            let relative_path = entry_path
                .strip_prefix("package/")
                .unwrap_or_else(|_| &entry_path); // Use original path if strip_prefix fails

            let dest_path = PathBuf::from(&dest_folder).join(relative_path);

            // Ensure the parent directory exists
            if let Some(parent_dir) = dest_path.parent() {
                std::fs::create_dir_all(parent_dir).expect("Failed to create parent directory");
            }

            // Unpack the entry to the adjusted destination path
            entry
                .unpack(&dest_path)
                .expect("Failed to unpack tar entry");
        });

    // Cleanup: Remove the temporary tar file
    std::fs::remove_file("temp.tar.gz").expect("Failed to remove temp file");

    println!("Tar file has been successfully downloaded and unpacked.");
}
//installer function that resolves remote packages and arranges to disk
fn package_installer(name: String, version: String) -> HashMap<String, Value> {
    // call download package from registry function with
    let resolved = resolve_remote_package(name, version).unwrap();
    // Now 'resolved' contains the parsed JSON data as a HashMap
    let dist = resolved.get("dist").unwrap();
    let version = resolved.get("version").unwrap();
    let tarball = dist.get("tarball").unwrap();
    let _integrity = dist.get("integrity").unwrap();
    let name = resolved.get("name").unwrap();
    // println!("the tar is {:?} and version is {:?}", tarball, version);
    println!("proceeding to install {}  version {}", name, version);
    extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
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
            println!("Dep object detected we should append to json");
            //update the dep object with installed package metadata
            update_dep_obj(json_file_data, name, version).unwrap();
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
