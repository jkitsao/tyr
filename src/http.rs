use crate::reconsole;
// use clap::error;
// use indicatif::ProgressBar;
use serde_json::Value;
use std::collections::HashMap;
use ureq::Error;
use url::form_urlencoded;
//remote registry path
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
//
pub fn resolve_remote_package(
    name: String,
    version: String,
) -> Result<HashMap<String, Value>, ureq::Error> {
    // println!("name is {}", name);
    // let trimmed = version.trim_matches("~");
    // let message = format!("Installing {}@{}", name, version);
    // console::show_info(message);
    let url_encoded_name: String = form_urlencoded::byte_serialize(name.as_bytes()).collect();
    let url = format!("{}/{}/{}", NPM_REGISTRY_URL, url_encoded_name, version);
    // println!("URL IS : {}", url);
    let res = ureq::get(&url)
        .set(
            "ALLOW",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        )
        .call();
    match res {
        Ok(response) => {
            /* it worked */
            let text = response.into_string().unwrap();
            let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
            Ok(map)
        }
        Err(Error::Status(_code, response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            // bar.finish();
            let text = response.into_string().unwrap();
            let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
            // eprintln!("Registry error code {:?} {:?}", code, text);
            // let error = String::from();
            reconsole::show_error(text);
            //return map with error values
            Ok(map)
        }
        Err(_) => {
            /* some kind of io/transport error */
            let error = String::from("Check your connection and try again");
            reconsole::show_error(error);
            // bar.finish();
            // let text = _.into_string().unwrap();
            // let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
            let mut map = HashMap::new();
            map.insert(
                "error".to_string(),
                serde_json::from_str("connection").unwrap(),
            );
            Ok(map)
        }
    }
}
