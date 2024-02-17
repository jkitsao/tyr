use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;
//handle scripts in package.json
pub fn parse_package_json(script: &str) -> Result<Value, serde_json::Error> {
    let path_name = format!("./node_tests/package.json");
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    let map: HashMap<String, Value> = serde_json::from_reader(reader).unwrap();
    let none = serde_json::to_value(String::from("")).unwrap();
    //
    match map.contains_key("scripts") {
        true => {
            let scripts: HashMap<String, Value> =
                serde_json::from_value(map.get("scripts").unwrap().clone()).unwrap();
            // dbg!(scripts);
            let runner = scripts.get(script).expect("Script not found ");
            // dbg!(runner);
            Ok(runner.clone())
        }
        false => Ok(none),
    }
}
pub fn execute_script(script: &str) -> Result<(), std::io::Error> {
    // println!("arg is {}", script.clone());
    //get the scripts value
    let exec = parse_package_json(script).expect("failed to get specified script in package.json");
    // Execute the command
    let output = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", exec.as_str().unwrap()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(exec.as_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    };

    // Handle output as needed (e.g., print to console)
    // println!("{}", String::from_utf8_lossy(&output.stdout));
    match output {
        Ok(output) => {
            if output.status.success() {
                io::stdout().write_all(&output.stdout).unwrap();
            } else {
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
        }
    }

    Ok(())
}
