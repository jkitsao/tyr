
// use ureq::Error;
use url::form_urlencoded;

use ureq::{Agent, Error, Request, Response};
// use ureq::{Response, Error, Error::Status};
use std::{result::Result, time::Duration, thread};
//remote registry path
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";

fn remove_non_numbers(input: &str) -> String {
    match input {
        "latest" => {
            "latest".to_string()
        }
        _ =>{
            let result: String = input
                .chars()
                .filter(|c| c.is_digit(10) || *c == '.')
                .collect();
            result
        }
    }
}

pub fn get_response(name: &str, version: &str) -> Result<Response, Error> {
    let url_encoded_name = form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>();
    let url = format!("{}/{}/{}", NPM_REGISTRY_URL, url_encoded_name, remove_non_numbers(version));
    // println!("URL: {}", url);

    let agent = Agent::new();
    let mut request = agent.get(&url).set("Accept", "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*");;

    // Set request headers
    // Perform the HTTP request with retries
    for _ in 0..5 {
        let response = request.clone().call();

        match response {
            Ok(response) => match response.status() {
                200 => return Ok(response),
                503 | 429 | 405 => {
                    if let Some(retry_secs) = response.header("retry-after").and_then(|h| h.parse().ok()) {
                        println!("Received {} status for {}, retrying in {} seconds", response.status(), url, retry_secs);
                        thread::sleep(Duration::from_secs(retry_secs));
                        continue;
                    }
                },
                _ => {
                    println!("Error: Failed to get response for {}", url);
                    thread::sleep(Duration::from_secs(5));
                }
            },
            Err(err) => {
                println!("Error: Failed to get response for {}: {}", url, err);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }

    Err(Error::Status(500,request.call().unwrap()))
}