/*

Install all the dependencies listed within
package.json in the local node_modules folder.

If tyr.lock is present and is enough to satisfy all the dependencies listed in package.json, the exact versions recorded in tyr.lock are installed, and tyr.lock will be unchanged. tyr will not check for newer versions.

If tyr.lock is absent, or is not enough to satisfy all the dependencies listed in package.json (for example, if you manually add a dependency to package.json), Yarn looks for the newest versions available that satisfy the constraints in package.json. The results are written to yarn.lock.

*/
// use std::collections::{HashMap, HashSet};
// use std::fs;
// use itertools::Itertools;
use std::{error::Error, fs};
use yarn_lock_parser::{parse_str, Entry};
// Step 1: Load Entries from Lockfile
pub fn load_entries_from_lockfile(lockfile_path: &str) {
    // Implement logic to read and parse the lockfile
    // Return a HashMap with dependency names as keys and versions as values
    // Example: {"dependency1": "1.2.3", "dependency2": "4.5.6", ...}
    // unimplemented!()
    let yarn_lock_text = fs::read_to_string(lockfile_path).unwrap();
    let entries: Vec<Entry> = parse_str(&yarn_lock_text).unwrap();

    // Extra elements are discarded
    // let hm: HashMap<Entry, Entry> = entries.into_iter().tuples();
    // let map  = entries
    // .array_chunks::<2>()
    // .map(|[k, v]| (k, v))
    // .collect::<HashMap<_, _>>();
    // hm
    println!("{:?}", entries);
}

// // Step 2: Read Manifest Files (package.json)
// fn read_manifest_files(manifest_path: &str) -> HashMap<String, String> {
//     // Implement logic to read and parse the manifest files
//     // Return a HashMap with dependency names as keys and versions as values
//     // Example: {"dependency1": "1.2.3", "dependency2": "4.5.6", ...}
//     // unimplemented!()
//     // let path_name = format!("./node_tests/package.json");
//     let file = fs::File::open(manifest_path).unwrap();
//     let reader = BufReader::new(file);
//     //
//     // let mut update = true;
//     // Read the JSON contents of the file and assign to Hashmap.
//     let json_file_data: HashMap<String, Value> = serde_json::from_reader(reader)?;
// }

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
//     let lockfile_path = "./node_tests/tyr.lock";
//     let manifest_path = "./node_tests/package.json";

//     let lockfile_entries = load_entries_from_lockfile(lockfile_path);
//     let manifest_entries = read_manifest_files(manifest_path);

//     let missing_entries = find_missing_entries(&lockfile_entries, &manifest_entries);

//     // Output or handle the missing entries as needed
//     println!("Missing or outdated dependencies: {:?}", missing_entries);
// }
