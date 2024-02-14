// use ureq::Error;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use ureq::{Agent, Error, Response};
use url::form_urlencoded;
// use ureq::{Response, Error, Error::Status};
use std::{result::Result, thread, time::Duration};
//remote registry path
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";

fn remove_non_numbers(input: &str) -> String {
    match input {
        "latest" => "latest".to_string(),
        _ => {
            let result: String = input
                .chars()
                .filter(|c| c.is_digit(10) || *c == '.')
                .collect();
            result
        }
    }
}

pub fn get_response(name: &str, version: &str) -> Result<Response, Error> {
    //check the version passed first
    if is_semver_number(version) {
        let _bar = ProgressBar::new(!0).with_prefix("Fetching...").with_style(
            ProgressStyle::default_spinner()
                .template("{prefix:>12.bright.cyan} {spinner} {msg:.cyan}")
                .unwrap(),
        );
        let url_encoded_name = form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>();
        let url = format!(
            "{}/{}/{}",
            NPM_REGISTRY_URL,
            url_encoded_name,
            remove_non_numbers(version)
        );
        // println!("URL: {}", url);

        let agent = Agent::new();
        let request = agent.get(&url).set(
            "Accept",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        );

        // Set request headers
        // Perform the HTTP request with retries
        for _ in 0..5 {
            let response = request.clone().call();

            match response {
                Ok(response) => {
                    //show
                    match response.status() {
                        200 => return Ok(response),
                        503 | 429 | 405 => {
                            if let Some(retry_secs) =
                                response.header("retry-after").and_then(|h| h.parse().ok())
                            {
                                println!(
                                    "Received {} status for {}, retrying in {} seconds",
                                    response.status(),
                                    url,
                                    retry_secs
                                );
                                thread::sleep(Duration::from_secs(retry_secs));
                                continue;
                            }
                        }
                        _ => {
                            let msg = format!("Error: Failed to get response for {}", url);
                            println!(" {}", style(msg).red().bright().blink());
                            thread::sleep(Duration::from_secs(5));
                        }
                    }
                }
                Err(err) => {
                    println!("{}", style(err).red().bright());
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }

        Err(Error::Status(500, request.call().unwrap()))
    } else {
        //fetch a package and a list of dependencies
        //we'll need to iter the deps and find a range that satisfies the aversion passed
        print!("version needs to be parsed differently for suree ***** theres an error bellow most likely \n");
        // let url_encoded_name = form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>();
        let url = format!(
            "{}/{}/",
            NPM_REGISTRY_URL,
            name,
            // remove_non_numbers(version)
        );
        println!("URL: {}", url);

        let agent = Agent::new();
        let request = agent.get(&url).set(
            "Accept",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        );

        // Set request headers
        // Perform the HTTP request with retries
        for _ in 0..5 {
            let response = request.clone().call();

            match response {
                Ok(response) => {
                    //show
                    match response.status() {
                        200 => {
                            // dbg!(&response.into_string().unwrap());
                            return Ok(response);
                        }
                        503 | 429 | 405 => {
                            if let Some(retry_secs) =
                                response.header("retry-after").and_then(|h| h.parse().ok())
                            {
                                println!(
                                    "Received {} status for {}, retrying in {} seconds",
                                    response.status(),
                                    url,
                                    retry_secs
                                );
                                thread::sleep(Duration::from_secs(retry_secs));
                                continue;
                            }
                        }
                        _ => {
                            let msg = format!("Error: Failed to get response for {}", url);
                            println!(" {}", style(msg).red().bright().blink());
                            thread::sleep(Duration::from_secs(5));
                        }
                    }
                }
                Err(err) => {
                    println!("{}", style(err).red().bright());
                    thread::sleep(Duration::from_secs(5));
                }
            }
        }

        Err(Error::Status(500, request.call().unwrap()))
    }
    //
}
//check and resolve semvar differently see what i mean below
fn is_semver_number(value: &str) -> bool {
    if value == "latest" {
        return true;
    }
    let semver_regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    semver_regex.is_match(value.trim_matches('"'))
}
