use ::serde_json::Value;
use ::std::collections::HashMap;
use nodejs_semver::{Range, Version};
pub fn split_package_version(name: &str) -> (String, String) {
    if name.starts_with('@') {
        // If the string starts with '@', skip the first '@'
        let v: Vec<&str> = name[1..].splitn(2, '@').collect();
        return (
            format!("@{}", v[0]),
            convert_to_version(v.get(1).map_or("", |&s| s)),
        );
    } else if name.contains('@') {
        // If the string doesn't start with '@', process as usual
        let v: Vec<&str> = name.splitn(2, '@').collect();
        return (
            v[0].to_string(),
            convert_to_version(v.get(1).map_or("", |&s| s)),
        );
    }

    // If there's no '@', assume the entire string is the package name
    (name.to_string(), String::from("latest"))
}
//convert single number string to semvar
fn convert_to_version(input: &str) -> String {
    // Check if the input string is a single integer
    if input.chars().all(|c| c.is_digit(10)) {
        // Convert the integer to a formatted string with ".0.0" appended
        return format!("{}.0.0", input);
    }

    // Return the input string as is if it doesn't meet the condition
    input.to_string()
}
//resolve semver range
pub fn resolve_semvar_range(
    input: &str,
    versions: HashMap<String, Value>,
) -> Result<HashMap<String, Value>, String> {
    //get the best version that satisfies the given input
    // Iterate over everything.
    // let e = extract_right_side(input);
    // println!("user input is: {e}");
    for (key, value) in &versions {
        let version: Version = key.parse().unwrap();
        let range: Range = extract_right_side(input).trim_matches('"').parse().unwrap();
        if version.satisfies(&range) {
            // println!("{key}");
            let data: HashMap<String, Value> = serde_json::from_value(value.clone()).unwrap();
            return Ok(data);
        }
    }
    Ok(versions)
}
fn extract_right_side(input_str: &str) -> String {
    // Check if the input string contains ':'
    if let Some(index) = input_str.find(':') {
        // Split the string by ':' and take the second part
        let right_side = &input_str[index + 1..].trim();
        return right_side.to_string();
    }

    // If the input string does not contain ':', return it as is
    input_str.to_string()
}
