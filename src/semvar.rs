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
            v.get(1).map_or("", |&s| s).to_string(),
        );
    } else if name.contains('@') {
        // If the string doesn't start with '@', process as usual
        let v: Vec<&str> = name.splitn(2, '@').collect();
        return (v[0].to_string(), v.get(1).map_or("", |&s| s).to_string());
    }

    // If there's no '@', assume the entire string is the package name
    (name.to_string(), String::from("latest"))
}
