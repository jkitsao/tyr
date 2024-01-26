// use clap::builder::Str;
// use clap::Error;
mod cli;
mod filesystem;
mod http;
mod init;
mod semvar;
mod unzip;
mod utils;
// use serde_json::json;
use std::collections::BTreeMap;
mod console;
use serde_json::Value;
use std::collections::HashMap;
// use std::env;
use std::fs;
use std::io::BufReader;
use std::path::Path;
// use std::path;
fn main() {
    // cli::get_args();
    cli::initialize_command_arguments()
}
//resolve dependancy/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided but i'll start
pub fn resolve_package_from_registry(dep: String, update: bool) {
    //get required values from the string
    let (name, version) = semvar::split_package_version(&dep);
    // println!("package is {}, and the version is {}", name, version);
    match version.as_ref() {
        "latest" => {
            // let message = format!("Querying NPM for {}", name);
            // console::show_success(message);
            // call package installer
            let package_metadata = package_installer(name.clone(), version);
            //create/update a lock file
            let res_package = filesystem::generate_lock_file(package_metadata).unwrap();
            //use the values returned to update package.json
            //also updates/creates dep in package.json
            //create or update dep
            filesystem::update_package_jason_dep(res_package, update).unwrap();
            //resolve next dependency
            resolve_next_dep(name);
        }
        //semvar string has been passed
        _ => {
            // version number has been passed
            // let message = format!("Querying NPM for {}", name);
            // console::show_success(message);
            // call package installer with semvar version
            let install_db = package_installer(name, version);
            filesystem::generate_lock_file(install_db).unwrap(); //also updates/creates dep in package.json
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
    // println!("proceeding to install {}  version {}", name, version);
    let message = format!("Installing {}@{}", name, version);
    console::show_info(message);
    unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
    resolved
}

//fetch next dep after instalation of package
//to resolve the next dependancies
fn resolve_next_dep(name: String) {
    let mut path_name = format!("./node_tests/node_modules/{}/package.json", name);
    let dir_name: String = format!("./node_tests/node_modules/{}", name);
    //check if pathname above exists
    if !Path::new(path_name.as_str()).exists() {
        // expect("Failed to create destination folder");
        utils::visit_dir(dir_name.clone()).unwrap();
        // handle the headache
        path_name = dir_name;
    }
    // let file = fs::File::open(path_name).unwrap();
    let file = fs::File::options()
        .read(true)
        .open(path_name)
        .expect("failed to create file");
    let reader = BufReader::new(file);
    // Read the JSON contents of the file and assign to Hashmap.
    let mut json_file_data: BTreeMap<String, Value> = serde_json::from_reader(reader).unwrap();
    //match to check id dep is available
    match json_file_data.contains_key("dependencies") {
        true => {
            let next_dep: Value = json_file_data.get_mut("dependencies").unwrap().clone();
            let temp_json: HashMap<String, String> = serde_json::from_value(next_dep).unwrap();
            let it = temp_json.iter();
            //check if theres dep
            match temp_json.is_empty() {
                true => {
                    // print!("********  no more dependencies *********");
                    let message = format!("Done ** ** 👍🏾");
                    // println!("****** installing the next one {:?}", package_name);
                    console::show_info(message);
                }
                false => {
                    // println!("{:?}", it);
                    // Iterate over the keys and values of the hashmap
                    for (key, value) in it {
                        // Remove backticks from the value
                        let new_value = value.replace('^', "");
                        // println!("Key: {}, Value: {}", key, value);
                        let package_name = format!("{}@{}", key, new_value);
                        // println!("{}", package_name);
                        //
                        // let message = format!("Installing *** {}", package_name);
                        // println!("****** installing the next one {:?}", package_name);
                        // console::show_info(message);
                        //pass false to prevent updating package json with resolved dep
                        resolve_package_from_registry(package_name.to_string(), false);
                    }
                    //format the map values to stringproceeding to install
                }
            }
        }
        //package.json does not contain dependency field
        false => {
            // println!("No dependencies in package")
            let message = format!("Done ****👍🏾");
            // println!("****** installing the next one {:?}", package_name);
            console::show_info(message);
        }
    }

    // check if dep has been installed
}
