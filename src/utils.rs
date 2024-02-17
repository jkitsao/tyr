// utility functions
use crate::semvar;
use ::serde_json::Value;
use ::std::collections::BTreeMap;
use nodejs_semver::{Range, Version};
use std::fs;
use std::io;
use std::io::{copy, BufReader};
use std::path::{Path, PathBuf};
//

// use std::io::;
// use std::path::PathBuf;
//helper function that walks  a directory
pub fn visit_dir(path_name: String) -> io::Result<String> {
    let mut entries = fs::read_dir(path_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.
    entries.sort();
    let paths: Vec<PathBuf> = entries.to_vec();
    let f = paths[0].to_str().unwrap();
    let file = String::from(f);
    // The entries have now been sorted by their path.
    Ok(file)
}
//check if dep has  been installed
//
pub fn should_resolve_dependency(package: String) -> bool {
    let (name, version) = semvar::split_package_version(&package.as_str());

    let dest_folder = format!("./node_modules/{}", name);
    //check if exact package has been installed
    // let b: bool = Path::new(dest_folder.as_str()).is_dir();
    if Path::new(dest_folder.as_str()).is_dir() {
        // check firt on file system
        let mut pckg_dest_folder = format!("./node_modules/{}/package.json", name);
        //check if there's an extra path inside first
        if !Path::new(pckg_dest_folder.as_str()).exists() {
            // expect("Failed to create destination folder");
            let new_path = visit_dir(dest_folder.clone()).unwrap();
            let read_path = format!("{}/package.json", new_path);
            // handle the headache
            pckg_dest_folder = read_path
        }
        //read the version
        let file = fs::File::open(pckg_dest_folder.clone()).unwrap();
        let reader = BufReader::new(file);
        // Read the JSON contents of the file and assign to Hashmap.
        let json_file_data: BTreeMap<String, Value> = serde_json::from_reader(reader).unwrap();
        // let (name, version) = semvar::split_package_version(&package.as_str());
        let installed_version = json_file_data.get("version");
        if let Some(v) = installed_version {
            //check to see if versions satisfy semver
            let local_version: Range = version.trim().trim_matches('"').parse().unwrap();
            let required_version: Version = v.to_string().trim().trim_matches('"').parse().unwrap();
            let satisfies = local_version.satisfies(&required_version);
            if satisfies {
                return false;
            }
            true
        } else {
            true
        }
    } else {
        true
    }
}
