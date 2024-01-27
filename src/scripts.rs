//handle scripts in package.json
pub fn parse_package_json(package_json_content: &str) -> Result<Value, serde_json::Error> {
    let path_name = format!("./node_tests/package.json");
    let file = fs::File::open(path_name).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_str(package_json_content)
}
fn execute_script(script: &str) -> Result<(), std::io::Error> {
    let output = Command::new("sh") // or "cmd" on Windows
        .arg("-c")
        .arg(script)
        .output()?;

    // Handle output as needed (e.g., print to console)
    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
