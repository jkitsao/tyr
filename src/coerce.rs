use nodejs_semver::{Range, Version};

pub fn coerce(version: &str, options: Option<&str>) -> Option<Version> {
    // Check if version is already a valid semantic version
    if let Ok(semver) = version.parse::<Version>() {
        return Some(semver);
    }

    // If version is a number, convert it to a string
    let version = if let Ok(number) = version.parse::<i64>() {
        number.to_string()
    } else {
        version.to_string()
    };

    // Check if version is a string
    if !version.chars().all(char::is_digit) {
        return None;
    }

    // Convert string to semantic version
    let match_result = if options.is_none() || options.unwrap().rtl.is_none() {
        // Match coercion regex for left-to-right coercion
        version
            .as_str()
            .match_indices(r"COERCION_REGEX_HERE")
            .next()
    } else {
        // Right-to-left coercion
        let mut match_result = None;
        let mut next;
        let mut regex = regex::Regex::new(r"COERCION_REGEX_RTL_HERE").unwrap();
        while let Some(matched) = regex.find(&version) {
            next = matched;
            if match_result.is_none() || next.end() != match_result.unwrap().end() {
                match_result = Some(next);
            }
            regex.set_match_start(next.end());
        }
        match_result
    };

    if let Some((start, end)) = match_result {
        let coerced_version = format!("{}.{}.{}", &version[start..end], &version[end..], "0");
        return Some(coerced_version.parse().unwrap());
    }

    None
}
