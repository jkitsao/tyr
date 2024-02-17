use clap::Error;
use serde_json::json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufWriter, Write};
//Generate lock files with package name,version,resolve url, and integrity checksum
pub fn generate_lock_file(
    package: HashMap<String, Value>,
    deps: BTreeMap<String, Value>,
) -> Result<HashMap<String, Value>, Error> {
    //model lock content
    struct LockFile {
        name: String,
        version: String,
        resolved: String,
        integrity: String,
        dependencies: Value,
    }
    //formatter function returns placeholders without double
    //quotes around the name, version, resolved, and integrity
    impl LockFile {
        fn format_for_lock_file(&self) -> String {
            format!(
                "{}: \n version {}\n  resolved {}\n  integrity {}\n dependencies \n {} \n \n",
                self.name, self.version, self.resolved, self.integrity, self.dependencies
            )
        }
    }
    //get required values from hashmap
    // dbg!(deps.clone());
    let dist = package.get("dist").unwrap();
    let version = package.get("version").unwrap();
    let tarball = dist.get("tarball").unwrap();
    let integrity = dist.get("integrity").unwrap();
    let name = package.get("name").unwrap();
    let dependencies: Value;
    //check to see is dep contains completed key
    if !deps.contains_key("status") {
        dependencies = json!(deps);
    } else {
        dependencies = json!({})
    }
    // dbg!(dependencies.clone());
    let mut path_name = "./tyr.lock".to_string();
    // fs::File::create(path)
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&mut path_name)
        .expect("failed to create a tyr.lock file");
    //construct a new lock object with package metadata
    let lock = LockFile {
        name: name
            .to_string()
            .replace("\"", "")
            .trim_matches('"')
            .to_string(),
        version: version.to_string().to_string(),
        integrity: integrity.to_string(),
        resolved: tarball.to_string(),
        dependencies,
    };
    write!(file, "{}", &lock.format_for_lock_file())?;

    Ok(package)
}
//update or create the dependencies on package.json after updating lock
//Basically updates dependency packages after installation
//first model the dep struct and package struct
pub fn update_package_jason_dep(package: HashMap<String, Value>, update: bool) -> io::Result<()> {
    //read contents of the file
    let path_name = "./package.json".to_string();
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
    //check if dependencies object is present on the data
    // let is_dep_init = json_file_data.contains_key("dependencies");
    match json_file_data.contains_key("dependencies") {
        true => {
            //update the dep object with installed package metadata
            update_dep_obj(json_file_data, name.clone(), version, update).unwrap();
        }
        false => {
            println!("Dep object not found we should create then add");
            // probably the first package
            create_dep_obj(json_file_data, name, version).unwrap();
        }
    }
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
    let dep_value: BTreeMap<String, Value> = serde_json::from_value(value).unwrap();
    //merge the 2 data structures
    metadata.extend(dep_value);
    let result = json!(metadata);
    let mut path_name = "./package.json".to_string();
    let file = fs::File::create(&mut path_name).expect("failed to create a package.json file");
    // write to package.json file
    let mut writer = BufWriter::new(file);
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
            //write output to file
            //serialize first
            let results = json!(metadata);
            let mut path_name = "./package.json".to_string();
            let file =
                fs::File::create(&mut path_name).expect("failed to create a package.json file");
            let mut writer = BufWriter::new(file);
            serde_json::to_writer_pretty(&mut writer, &results)?;
            Ok(name)
        }
        false => {
            // dependency object wont be updated
            Ok("error".to_string())
        }
    }
}
fn _combine_dependency_and_version(input: &str) -> String {
    let mut parts = input.splitn(2, '@'); // Split at the first '@'

    if let (Some(name), Some(version)) = (parts.next(), parts.next()) {
        format!("{}@{}", name.trim(), version.trim())
    } else {
        // Handle the case where there's no '@' or missing parts
        input.to_string()
    }
}
