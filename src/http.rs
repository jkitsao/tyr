use serde_json::Value;
use std::collections::HashMap;
// use std::time::Duration;
use ureq::Error;
// use url::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use url::form_urlencoded;
//remote registry path
const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";
//
pub fn resolve_remote_package(
    name: String,
    version: String,
) -> Result<HashMap<String, Value>, ureq::Error> {
    println!("name is {}", name);
    let url_encoded_name: String = form_urlencoded::byte_serialize(name.as_bytes()).collect();
    let url = format!("{}/{}/{}", NPM_REGISTRY_URL, url_encoded_name, version);
    println!("URL IS : {}", url);
    match ureq::get(&url)
        .set(
            "ALLOW",
            "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        )
        .call()
    {
        Ok(response) => {
            /* it worked */
            let text = response.into_string().unwrap();
            let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
            Ok(map)
        }
        Err(Error::Status(code, response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            let text = response.into_string().unwrap();
            let map: HashMap<String, Value> = serde_json::from_str(&text.as_ref()).unwrap();
            eprintln!("Registry error code {:?} {:?}", code, text);
            //return map with error values
            Ok(map)
        }
        Err(_) => {
            /* some kind of io/transport error */
            eprintln!("connection has been interrupted");
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
// url resolver
