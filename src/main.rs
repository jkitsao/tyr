// use clap::builder::Str;
// use clap::Error;
mod cli;
mod filesystem;
mod http;
mod init;
mod install;
mod semvar;
mod unzip;
mod utils;
// use serde_json::json;
use std::collections::BTreeMap;
mod reconsole;


use serde_json::Value;
use std::collections::HashMap;
// use std::env;
use console::{style, Emoji};
use indicatif::HumanDuration;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
// use std::path;
//for printing to the screen
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static _CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static _PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
//update structure
struct JsonFile {
    should_update:bool
}
fn main() {
    // cli::get_args();
    let started = Instant::now();
    cli::initialize_command_arguments();
    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}
//resolve dependency/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided, but I'll start
pub fn resolve_package_from_registry(dep: String,update:bool) {
    // let mut update=true;
    //get required values from the string
    let (name, version) = semvar::split_package_version(&dep);
    // println!("package is {}, and the version is {}", name, version);
    //     let mut update =  JsonFile {
    //     should_update:true
    // };
    match version.as_ref() {
        "latest" => {
            println!(
                "{} {}Resolving packages...",
                style("[1/4]").bold().dim(),
                LOOKING_GLASS
            );

            let next_deps = package_installer(name.clone(), version,update.clone());
            // update.should_update=false;
            // update=false;

            // const CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
            // println!(
            //     "{} {}",
            //     CLIP,
            //     style("peer dependency").italic().bright().green()
            // );
            if let Some((key, value)) = next_deps.iter().next() {
                // update.should_update=false;
                // is_update=false;
                let result = format!("{}@{}", key, value);
                //don't update package.json
                // println!("the result is {}", result);//
                resolve_package_from_registry(result,false);
            } else {
                println!("BTreeMap is empty");
            }
            //resolve next dependency
            // resolve_next_dep(name.clone());
        }
        //semvar string has been passed
        _ => {

            // const CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
            // println!(
            //     "{} {}",
            //     CLIP,
            //     style("peer dependency").italic().bright().green()
            // ); //semver as version
            // resolve_next_dep(name);
            let next_deps = package_installer(name.clone(), version,update.clone());
            // update.should_update=false;

            // update=false;
            match next_deps.iter().next() {
                Some((key, value)) => {
                    // update.should_update=false;
                    // is_update=false;
                    let result = format!("{}@{}", key, value);
                    //dont update package.json

                    // println!(" the result is {}", result);
                    resolve_package_from_registry(result,false);
                }
                None => {
                    println!("BTreeMap is empty");
                }
            }

        }
    }
}

// install package to disk
//installer function that resolves remote packages and arranges to disk
// static  mut is_update=true;

fn package_installer(name: String, version: String, mut update:bool) -> BTreeMap<String, Value> {
    // TODO: have it return the next dep to be resolved
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
    println!(
        "{} {}Fetching {}@{}",
        style("[2/4]").bold().dim(),
        TRUCK,
        &name,
        version
    );
    //Download and extract to file
    let deps = unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
    //create/update a lock file
    let res_package = filesystem::generate_lock_file(resolved.clone(),deps.clone()).unwrap();
    //use the values returned to update package.json
    //also updates/creates dep in package.json
    //create or update dependencies
    // println!("Next deps is ..... are we updating  {:?} {:?}",deps,is_update);
    filesystem::update_package_jason_dep(res_package, update).unwrap();
    // is_update=false;
    // is_update=false;
    // println!("Next deps is ..... are we updating  {:?} {:?}",deps,is_update);
    deps
}

//fetch next dep after installation of package
//to resolve the next dependencies
// fn resolve_next_dep(name: String) {
//     println!("Moving on to {}",name);
//     let mut path_name = format!("./node_tests/node_modules/{}/package.json", name);
//     let dir_name: String = format!("./node_tests/node_modules/{}", name);
//     // check if pathname above exists
//     // some packages i.e busboy come with an extra directory in the root
//     // in which case read the extra directory to get package.json
//     if !fs::metadata(&path_name).is_ok() {
//         println!("dir {}",path_name.clone());
//         // expect("Failed to create destination folder");
//         //check if directory exists
//         // let dir_exists = fs::metadata(dir_name)
//         //     .map(|metadata| metadata.is_dir())
//         //     .unwrap_or(false);
//         // if dir_exists {
//         //     println!("Checking the dir step");
//         //     //
//             let new_path = utils::visit_dir(format!("./node_tests/node_modules/{}", name)).unwrap();
//             let read_path = format!("{}/package.json", new_path);
//         //     // handle the headache
//             path_name = read_path
//         // }
//     } else {
//         // let file = fs::File::open(path_name).unwrap();
//         let file = fs::File::options()
//             .read(true)
//             .open(path_name)
//             .expect("failed to create file");
//         let reader = BufReader::new(file);
//         // Read the JSON contents of the file and assign to Hashmap.
//         let mut json_file_data: BTreeMap<String, Value> = serde_json::from_reader(reader).unwrap();
//         //match to check id dep is available
//         match json_file_data.contains_key("dependencies") {
//             true => {
//                 let next_dep: Value = json_file_data.get_mut("dependencies").unwrap().clone();
//                 let temp_json: HashMap<String, String> = serde_json::from_value(next_dep).unwrap();
//                 let it = temp_json.iter();
//                 //check if there's dep
//                 match temp_json.is_empty() {
//                     true => {
//                         // print!("********  no more dependencies *********");
//                         // let message = format!("Done ** ** 👍🏾");
//                         // println!("****** installing the next one {:?}", package_name);
//                         // console::show_info(message);
//                     }
//                     false => {
//                         // println!("{:?}", it);
//                         // Iterate over the keys and values of the hashmap
//                         for (key, value) in it {
//                             // Remove backticks from the value
//                             let new_value = value.replace('^', "");
//                             // println!("Key: {}, Value: {}", key, value);
//                             let package_name = format!("{}@{}", key, new_value);
//                             // println!("{}", package_name);
//                             //
//                             // let message = format!("Installing *** {}", package_name);
//                             println!("****** installing the next one {:?}", package_name);
//                             // console::show_info(message);
//                             //pass false to prevent updating package json with resolved dep
//                             resolve_package_from_registry(package_name.to_string(), false);
//                         }
//                         //format the map values to string proceeding to install
//                     }
//                 }
//             }
//             //package.json does not contain dependency field
//             false => {
//                 // println!("No dependencies in package")
//
//                 // let message = format!("Done ****👍🏾");
//                 // println!("****** installing the next one {:?}", package_name);
//                 // console::show_info(message);
//             }
//         }
//     }
//
//     // check if dep has been installed
// }
