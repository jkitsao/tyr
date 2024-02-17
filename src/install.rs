/*

Install all the dependencies listed within
package.json in the local node_modules folder.

If tyr.lock is present and is enough to satisfy all the dependencies listed in package.json, the exact versions recorded in tyr.lock are installed, and tyr.lock will be unchanged. tyr will not check for newer versions.

If tyr.lock is absent, or is not enough to satisfy all the dependencies listed in package.json (for example, if you manually add a dependency to package.json), Yarn looks for the newest versions available that satisfy the constraints in package.json. The results are written to yarn.lock.

*/
use std::collections::{HashMap, HashSet};
// use std::collections::HashMap;
// use crate::resolve_package_from_registry;
// use std::fs;
// use itertools::Itertools;
use std::fs;
// use  crate::resolve_package_from_registry;
// use yarn_lock_parser::{parse_str, Entry};
use serde_json;
use serde_json::Value;
use std::io::BufReader;
// use crate::resolve_package_from_registry;

// Step 1: Load Entries from Lockfile
pub fn load_entries_from_lockfile(lockfile_path: &str) {
    // Implement logic to read and parse the lockfile
    // Return a HashMap with dependency names as keys and versions as values
    // Example: {"dependency1": "1.2.3", "dependency2": "4.5.6", ...}
    //read package json file metadata
    let path_name = "./package.json".to_string();
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    let json_file_data: HashMap<String, Value> = serde_json::from_reader(reader).unwrap();
    //get dependency from json structure
    match json_file_data.contains_key("dependencies") {
        //if  not true dependency object is not available
        true => {
            let value = json_file_data.get("dependencies").unwrap();
            let json_deps: HashMap<String, Value> = serde_json::from_value(value.clone()).unwrap();
            //format and flatten to a vec string@version
            let flattened_json_packages: Vec<String> = json_deps
                .iter()
                .map(|(key, value)| {
                    if let Value::String(version) = value {
                        format!("{}@{}", key, version)
                    } else {
                        format!("{}@UNKNOWN", key)
                    }
                })
                .collect();

            // If you specifically need Vec<&str>, you can map the formatted strings into &str.
            // let flattened_json_packages: Vec<&str> = formatted_deps.iter().map(|s| s.as_str()).collect();
            // parse lock file
            let lock_file_text = fs::read_to_string(lockfile_path).unwrap();
            // Split input into lines and filter out empty lines
            let packages = parse_lock_file(lock_file_text.as_str());
            //remove non string char from packages
            // let valid_
            // let to_json=serde_json::from_value(packages);
            //flatten lock file deps to a vec also of string@version
            let flattened_lock_packages = flatten_packages(&packages);
            // println!("flattened package json: {:?} and locks are {:?}",flattened_json_packages,flattened_lock_packages);
            //turn both values to sets and compare differences
            let lock_file_set: HashSet<String> = flattened_lock_packages.into_iter().collect();
            let json_data_set: HashSet<String> = flattened_json_packages.into_iter().collect();
            // Symmetric difference of hashsets
            let results: HashSet<&String> =
                json_data_set.symmetric_difference(&lock_file_set).collect();
            println!("The sym difference is: {:?}", results)
            // let result =Vec::from_iter(res);
            //
            // for pckg in results {
            //     resolve_package_from_registry(pckg.to_string(),false)
            // }
        }
        false => {
            println!("Cannot find dependencies to install, Check your package.json file")
        }
    }
}
fn remove_non_numbers(input: &str) -> String {
    let result: String = input
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.')
        .collect();
    result
}

// // Step 3: Internal Algorithm to Identify Missing Entries
// fn find_missing_entries(
//     lockfile_entries: &HashMap<String, String>,
//     manifest_entries: &HashMap<String, String>,
// ) -> HashSet<String> {
//     // Implement logic to compare entries and find missing dependencies
//     // Return a HashSet of dependency names that are missing or need updates
//     // Example: {"missing_dependency1", "outdated_dependency2", ...}
//     // unimplemented!()
//     println("lockfile content is {}", lockfile_entries);
//     println("manifest content is {}", manifest_entries);
// }

// // Example Usage
// fn install() {
//     let lockfile_path = "./tyr.lock";
//     let manifest_path = "./package.json";

//     let lockfile_entries = load_entries_from_lockfile(lockfile_path);
//     let manifest_entries = read_manifest_files(manifest_path);

//     let missing_entries = find_missing_entries(&lockfile_entries, &manifest_entries);

//     // Output or handle the missing entries as needed
//     println!("Missing or outdated dependencies: {:?}", missing_entries);
// }

#[derive(Debug)]
struct Package {
    version: String,
    resolved: String,
    integrity: String,
    dependencies: HashMap<String, String>,
}

fn parse_lock_file(lock_file_content: &str) -> HashMap<String, Package> {
    let mut packages = HashMap::new();
    let mut current_package = String::new();
    let mut current_package_data = Vec::new();

    for line in lock_file_content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }
        if trimmed_line.ends_with(':') {
            if !current_package_data.is_empty() {
                let package = parse_package_data(&current_package_data.join("\n"));
                packages.insert(current_package.clone(), package);
                current_package_data.clear();
            }
            current_package = trimmed_line[..trimmed_line.len() - 1].to_string();
        // Remove trailing colon
        } else {
            current_package_data.push(trimmed_line.to_string());
        }
    }

    if !current_package_data.is_empty() {
        let package = parse_package_data(&current_package_data.join("\n"));
        packages.insert(current_package.clone(), package);
    }

    packages
}

fn parse_package_data(data: &str) -> Package {
    let mut version = String::new();
    let mut resolved = String::new();
    let mut integrity = String::new();
    let mut dependencies = HashMap::new();

    let mut parsing_dependencies = false;
    for line in data.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }
        if trimmed_line.starts_with("dependencies") {
            parsing_dependencies = true;
            continue;
        }
        if parsing_dependencies {
            if let Ok(deps) = serde_json::from_str::<HashMap<String, String>>(trimmed_line) {
                dependencies.extend(deps);
                for (_, value) in dependencies.iter_mut() {
                    // Remove non-number characters from the value
                    *value = value
                        .chars()
                        .filter(|c| c.is_digit(10) || *c == '.')
                        .collect();
                }
            }
        } else {
            let parts: Vec<_> = trimmed_line.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[0] {
                    "version" => version = remove_non_numbers(parts[1]),
                    "resolved" => resolved = parts[1].to_string(),
                    "integrity" => integrity = parts[1].to_string(),
                    _ => {}
                }
            }
        }
    }

    Package {
        version,
        resolved,
        integrity,
        dependencies,
    }
}

fn flatten_packages(packages: &HashMap<String, Package>) -> Vec<String> {
    let mut flattened_packages = Vec::new();

    for (package_name, package) in packages {
        let package_str = format!("{}@{}", package_name, package.version);
        flattened_packages.push(package_str);

        for (dependency_name, dependency_version) in &package.dependencies {
            let dependency_str = format!("{}@{}", dependency_name, dependency_version);
            flattened_packages.push(dependency_str);
        }
    }

    flattened_packages
}
