use crate::reconsole;
// use clap::error;
// use indicatif::ProgressBar;
use serde_json::Value;
use std::collections::{HashMap,BTreeMap};
// use ureq::Error;
use url::form_urlencoded;

use ureq::{Agent, Error, Request, Response};
// use ureq::{Response, Error, Error::Status};
use std::{result::Result, time::Duration, thread};
//remote registry path
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
//
// pub fn resolve_remote_package(
//     name: String,
//     version: String,
// ) -> Result<Response, Error> {
//     let url_encoded_name: String = form_urlencoded::byte_serialize(name.as_bytes()).collect();
//     // let url_encoded_version: String = form_urlencoded::byte_serialize(version.as_bytes()).collect();
//     let agent: Agent = AgentBuilder::new()
//         .build();
//     let url = format!("{}/{}/{}", NPM_REGISTRY_URL, url_encoded_name, remove_non_numbers(version.as_str()));
//     println!("URL IS : {}", url);
//     let res = agent.get(&url)
//         .set(
//             "ALLOW",
//             "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
//         )
//         .call();
//     match res {
//         Ok(response) => {
//             /* it worked */
//             let text = response.into_string().unwrap();
//             let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
//             Ok(map)
//         }
//         Err(Error::Status(_code, response)) => {
//             /* the server returned an unexpected status
//             code (such as 400, 500 etc) */
//             // bar.finish();
//             let text = response.into_string().unwrap();
//             let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
//             // eprintln!("Registry error code {:?} {:?}", code, text);
//             let bmap=map.into_iter().collect();
//             // let error = String::from();
//             // reconsole::show_error(text);
//             //return map with error values
//             Err(bmap)
//         }
//         Err(response) => {
//             /* some kind of io/transport error */
//             // let error = String::from("Check your connection and try again");
//             // reconsole::show_error(error);
//             // // bar.finish();
//             // // let text = _.into_string().unwrap();
//             // // let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
//             // let mut map = HashMap::new();
//             // map.insert(
//             //     "error".to_string(),
//             //     serde_json::from_str("connection").unwrap(),
//             // );
//             // let bmap=map.into_iter().collect();
//             // Err(bmap)
//         }
//     }
// }
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
    println!("URL: {}", url);

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