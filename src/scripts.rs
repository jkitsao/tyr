use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
// use std::io::{self, Write};
use std::process::Command;
use std::process::Stdio;
//handle scripts in package.json
pub fn parse_package_json(script: &str) -> Result<Value, serde_json::Error> {
    let path_name = format!("./package.json");
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
    let child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(exec.as_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(exec.as_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    };

    // Handle the child process
    match child {
        Ok(mut child) => {
            // Read and print stdout and stderr of the child process
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            // Print output lines from a stream
            fn print_output<T: BufRead>(stream: T) {
                for line in stream.lines() {
                    if let Ok(line) = line {
                        println!("{}", line);
                    }
                }
            }

            // Print stdout and stderr in separate threads
            let stdout_thread = std::thread::spawn(move || print_output(BufReader::new(stdout)));
            let stderr_thread = std::thread::spawn(move || print_output(BufReader::new(stderr)));

            // Wait for the child process to finish
            let _ = child.wait();

            // Wait for the threads to finish
            let _ = stdout_thread.join();
            let _ = stderr_thread.join();
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
        }
    }

    Ok(())
}
