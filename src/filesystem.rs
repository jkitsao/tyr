use serde_json::Value;
use std::collections::HashMap;
// use std::fs;
use clap::Error;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
// use std::io::Write;
use std::io;
use std::io::{BufReader, BufWriter, Write};
//Generate lock files with package name,version,resolve url, and integrity checksum
pub fn generate_lock_file(
    package: HashMap<String, Value>,
    deps:BTreeMap<String,Value>
) -> Result<HashMap<String, Value>, Error> {
    //model lock content
    struct LockFile {
        name: String,
        version: String,
        resolved: String,
        integrity: String,
        dependencies:Value
    }
    //formatter function returns placeholders without double quotes around the name, version, resolved, and integrity
    impl LockFile {
        fn format_for_lock_file(&self) -> String {
            format!(
                "{}: \n version {}\n  resolved {}\n  integrity {}\n dependencies \n {} \n \n",
                self.name, self.version, self.resolved, self.integrity,self.dependencies
            )
        }
    }
    //get required values from hashmap
    let dist = package.get("dist").unwrap();
    let version = package.get("version").unwrap();
    let tarball = dist.get("tarball").unwrap();
    let integrity = dist.get("integrity").unwrap();
    let name = package.get("name").unwrap();
    // let contains_dependencies = deps.contains_key("dependencies");
    let mut dependencies= json!(deps);
    //
    // if contains_dependencies {
    //     dependencies= json!(deps.get("dependencies").unwrap())
    // }

    // let next_deps = deps.get("dependencies").expect("Cannot get deps from next pckg");
    //create a lock file with fs package and write to it
    let mut path_name = "./node_tests/tyr.lock".to_string();
    // fs::File::create(path)
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mut path_name)
        .expect("failed to create a package.lock file");
    //construct a new lock object with package metadata
    let lock = LockFile {
        name: name
            .to_string()
            .replace("\"", "")
            .trim_matches('"')
            .to_string(),
        version: version
            .to_string()
            .replace("\"", "")
            .trim_matches('"')
            .to_string(),
        integrity: integrity.to_string(),
        resolved: tarball.to_string(),
        dependencies
    };
    // let name = format!("{}@{}", name, version);
    // let dep_name = combine_dependency_and_version(&name);
    // let formatted = lock.format_for_lock_file();
    write!(file, "{}", &lock.format_for_lock_file())?;
    // println!("Saved lockfile");
    // update_package_jason_dep(package).unwrap();

    Ok(package)
}
//update or create the dependencies on package.json after updating lock
//function to update dependency packages after installation
//first model the dep struct and package struct
pub fn update_package_jason_dep(package: HashMap<String, Value>, update: bool) -> io::Result<()> {
    //read contents of the file
    let path_name = "./node_tests/package.json".to_string();
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    //
    // let mut update = true;
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
    //check if dependencies object is present on the data
    // let is_dep_init = json_file_data.contains_key("dependencies");
    match json_file_data.contains_key("dependencies") {
        true => {
            // println!("Dep object detected we should append to json");
            //update the dep object with installed package metadata
            update_dep_obj(json_file_data, name.clone(), version, update).unwrap();
            // update = false;
            // resolve_next_dep(name.to_string());
            // println!("current boolean value {} ", update);
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
    let mut path_name = "./node_tests/package.json".to_string();
    let file = fs::File::create(&mut path_name).expect("failed to create a package.json file");
    // write to package.json file
    let mut writer = BufWriter::new(file);
    // fs::write(&mut path_name, b"Lorem ipsum").expect("failed to write to package.json file");
    serde_json::to_writer_pretty(&mut writer, &result)?;
    writer.flush()?;
    Ok(())
}

//update the dependency object
// static mut update_dep: bool = true;
fn update_dep_obj(
    mut metadata: BTreeMap<String, Value>,
    name: String,
    version: String,
    update: bool,
) -> io::Result<String> {
    //check if we need to update dependencies
    match update {
        true => {
            // create the json value with serde
            let current_dep: Value = metadata.get_mut("dependencies").unwrap().clone();
            //append installed package meta on the current_dep value
            let mut temp_json: HashMap<String, String> =
                serde_json::from_value(current_dep).unwrap();
            temp_json.insert(name.clone(), version);
            //update package.json instance with new dependencies
            if let Some(x) = metadata.get_mut("dependencies") {
                *x = json!(temp_json);
            };
            // println!("metadata {:?}", metadata);
            //write output to file
            //serialize first
            let results = json!(metadata);
            let mut path_name = "./node_tests/package.json".to_string();
            let file =
                fs::File::create(&mut path_name).expect("failed to create a package.json file");
            let mut writer = BufWriter::new(file);
            serde_json::to_writer_pretty(&mut writer, &results)?;
            Ok(name)
        }
        false => {
            // println!("not updating dep");
            // dependency object wont be updated
            Ok("error".to_string())
        }
    }
}
fn combine_dependency_and_version(input: &str) -> String {
    let mut parts = input.splitn(2, '@'); // Split at the first '@'

    if let (Some(name), Some(version)) = (parts.next(), parts.next()) {
        format!("{}@{}", name.trim(), version.trim())
    } else {
        // Handle the case where there's no '@' or missing parts
        input.to_string()
    }
}
