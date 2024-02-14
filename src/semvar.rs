// use nodejs_semver::Version;
// use regex::Regex;
//resolve semver versions from the name
//
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
//
