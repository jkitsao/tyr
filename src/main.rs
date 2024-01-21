// use clap::builder::Str;
// use clap::Error;
// use clap::builder::Str;
// use reqwest;
// use reqwest::header::ALLOW;
// use reqwest::Client;
use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};
use serde_json::json;
// use std::collections::HashMap;
use flate2::read::GzDecoder; // Add this import for Gzip support
use serde_json::{from_slice, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::copy;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
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
        "main":project.entry_point,
        "license": project.license
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
            // call download package from registry function with
            let resolved = resolve_remote_package(name, version).unwrap();
            // Now 'resolved' contains the parsed JSON data as a HashMap
            let dist = resolved.get("dist").unwrap();
            let version = resolved.get("version").unwrap();
            let tarball = dist.get("tarball").unwrap();
            let _integrity = dist.get("integrity").unwrap();
            let name = resolved.get("name").unwrap();
            // println!("the tar is {:?} and version is {:?}", tarball, version);
            // println!("proceeding to install {}",name);
            extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());

            //latest as version
        }
        _ => {
            let semvar_version = Version::parse(&version).unwrap();
            let res = resolve_full_version(semvar_version);
            // let map: HashMap<String, Value> = serde_json::from_value(&semvar_version).unwrap();
            println!("package is {:?} semvar {:?}", name, res);
            // resolve_remote_package(name, "latest".to_string()).unwrap();

            // call download package from registry function with
            //semver as version
        }
    }
    // let mut path_name = format!("./{}/package.json", project.name);
    // let req = VersionReq::parse(&dep).unwrap();
    // println!("Package to install {} semvar version {:?}", name, version);
    // Ok(())
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
    // let profile = map.get("dist").unwrap();
    // Now 'map' contains the parsed JSON data as a HashMap
    // println!("the url is {:#?}", url);
    // let dist = map.get("dist").unwrap();
    // let version = map.get("version").unwrap();
    // let tar = dist.get("tarball").unwrap();
    // println!("the tar is {:?} and version is {:?}", tar, version);
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
// fn extract_tarball(bytes: Bytes, dest: String) -> Result<(), Error> {
//     let bytes = &bytes.to_vec()[..];
//     let gz = GzDecoder::new(bytes);
//     let mut archive = Archive::new(gz);

//     // All tarballs contain a /package directory to the module source, this should be removed later to keep things as clean as possible
//     archive.unpack(&dest);
//     Ok(())
// }
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
    archive
        .entries()
        .expect("Failed to get tar entries")
        .filter_map(|entry| {
            // Filter entries under "/package" and adjust the extraction path
            let entry = entry.expect("Failed to get tar entry");
            let entry_path = entry.path().expect("Failed to get entry path");
            if entry_path.starts_with("package/") {
                Some(entry)
            } else {
                None
            }
        })
        .for_each(|mut entry| {
            let dest_path = {
                let path = entry.path().expect("Failed to get entry path");
                let suffix = path
                    .strip_prefix("package/")
                    .expect("Failed to strip prefix");
                PathBuf::from(&dest_folder).join(suffix)
            };

            entry
                .unpack(&dest_path)
                .expect("Failed to unpack tar entry");
        });

    // Cleanup: Remove the temporary tar file
    std::fs::remove_file("temp.tar.gz").expect("Failed to remove temp file");

    println!("Tar file has been successfully downloaded and unpacked.");
}
