use ctrlc;
use owo_colors::OwoColorize;
mod cli;
mod filesystem;
mod http;
mod init;
mod install;
mod scripts;
mod semvar;
mod unzip;
mod utils;
use std::collections::{BTreeMap, HashMap};
mod banner;
mod reconsole;
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_json::Value;
use std::process::{self, ExitCode};
// use std::time::Instant;
static _LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static _TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static _CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static _SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
//update structure

fn main() -> ExitCode {
    // graceful shutdown
    ctrlc::set_handler(move || {
        println!(
            "{}",
            style(
                "Received interrupt signal (Ctrl+C). Gracefully shutting down...
    "
            )
            .bold()
            .yellow()
            .bright()
        );
        process::exit(0)
        // return ControlFlow::Break();
        //exit with code
    })
    .expect("Error setting Ctrl-C handler");
    //
    cli::initialize_command_arguments();
    ExitCode::SUCCESS
}
//resolve dependency/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided, but I'll start
pub fn resolve_package_from_registry(dep: String, update: bool) {
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    // get the package name and version from user arg
    let (name, version) = semvar::split_package_version(&dep);
    //call installer function
    let next_deps = package_installer(name.clone(), version, update.clone());
    //check next deps for "status" key and "completed" value
    //if available no more deps are present
    //iterate over next deps
    if let Ok(dependencies) = next_deps {
        let pb = ProgressBar::new(dependencies.len() as u64);
        let m = MultiProgress::new();
        let mut count = 1;
        for (key, value) in dependencies.iter() {
            if key.clone() != String::from("status") {
                let result = format!("{}@{}", key, value);
                let f = format!("[+]{}✅", result.clone());
                pb.set_style(spinner_style.clone());
                pb.set_prefix(format!("[{}/{}]", count.clone(), dependencies.len()));
                // Don't update package.json
                // println!("the result is {}", result);
                //check if package has been resolved first and use that
                let should_install = utils::should_resolve_dependency(result.clone());
                if should_install {
                    pb.set_message(format!("{}", style(f).bright().bright_green()));
                    pb.inc(1);
                    resolve_package_from_registry(result, false);
                    pb.finish_and_clear();
                    count += 1;
                }
            }
        }
        m.clear().unwrap();
    } else if let Err(err) = next_deps {
        println!("{} {}", PAPER, style(err).bright().yellow());
    }
}

//Installs remote packages and arranges to disk(Node_modules)
fn package_installer(
    name: String,
    version: String,
    update: bool,
) -> Result<BTreeMap<String, Value>, String> {
    // call download package from registry function with package name and version
    let res = http::get_response(name.as_str(), version.as_str());
    //Match for response resolved or error
    match res {
        Ok(response) => {
            // Now 'resolved' contains the parsed JSON data as a HashMap
            let text_response: String = response.into_string().unwrap();
            let resolved: HashMap<String, Value> =
                serde_json::from_str(&text_response.as_ref()).unwrap();
            let pckg_data: Value;
            /*
             * below we have to option from the response and into resolved
             * one resolved with dist the latter with versions. for dist continue normally for versions
             * resolve range that satisfies
             */
            //deal with dist first
            let pckg_has_versions = resolved.contains_key("versions");
            let _pckg_dist = resolved.contains_key("dist");
            //get versions
            if pckg_has_versions {
                let versions: HashMap<String, Value> =
                    serde_json::from_value(resolved.get("versions").unwrap().clone()).unwrap();
                let data = semvar::resolve_semvar_range(version.clone().as_str(), versions.clone());
                //update pack_data to equal semver_resolve return value
                match data {
                    Ok(res) => {
                        let data = res.get("dist").unwrap().clone();
                        pckg_data = data;
                    }
                    Err(err) => {
                        eprint!("{err}");
                        pckg_data = resolved.get("dist").unwrap().clone();
                    }
                }
            } else {
                //update pckg_data to dist returned
                pckg_data = resolved.get("dist").unwrap().clone();
            }
            let _pckg_map: HashMap<String, Value> =
                serde_json::from_value(pckg_data.clone()).unwrap();
            // println!("resolved: {:?}", pckg_data);
            let next_deps = if resolved.contains_key("dependencies") {
                let res: BTreeMap<String, Value> =
                    serde_json::from_value(resolved.get("dependencies").unwrap().to_owned())
                        .unwrap();
                res
            } else {
                if resolved.contains_key("versions") {
                    // println!("results: {:?}", resolved);
                    let versions: HashMap<String, Value> =
                        serde_json::from_value(resolved.get("versions").unwrap().clone()).unwrap();
                    let data =
                        semvar::resolve_semvar_range(version.clone().as_str(), versions.clone())
                            .unwrap();
                    // dbg!(data.clone());
                    //check if dep object is present
                    let is_deps = data.contains_key("dependencies");
                    match is_deps {
                        true => {
                            let b: BTreeMap<String, Value> =
                                serde_json::from_value(data.get("dependencies").unwrap().clone())
                                    .unwrap();
                            b
                        }
                        false => {
                            // Create a BTreeMap with String keys and Value values
                            let mut btree: BTreeMap<String, Value> = BTreeMap::new();
                            btree.insert("status".to_string(), serde_json::json!("completed"));
                            btree
                        }
                    }
                } else {
                    // Create a BTreeMap with String keys and Value values
                    let mut btree: BTreeMap<String, Value> = BTreeMap::new();
                    btree.insert("status".to_string(), serde_json::json!("completed"));
                    btree
                }
            };
            // dbg!(next_depss.unwrap().clone());
            let dist = pckg_data.clone();
            let tarball = dist.get("tarball").unwrap();
            let name = resolved.get("name").unwrap();
            //Download and extract to file
            let _deps =
                unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
            let _pckg_map: HashMap<String, Value> =
                serde_json::from_value(pckg_data.clone()).unwrap();
            //check value of resolve first if it has versions parse
            //else
            if resolved.contains_key("versions") {
                let versions: HashMap<String, Value> =
                    serde_json::from_value(resolved.get("versions").unwrap().clone()).unwrap();
                let data = semvar::resolve_semvar_range(version.clone().as_str(), versions.clone())
                    .unwrap();
                // let b: BTreeMap<String, Value> = data.into_iter().collect();
                let res_package = filesystem::generate_lock_file(data, next_deps.clone()).unwrap();
                //use the values returned to update package.json
                filesystem::update_package_jason_dep(res_package, update).unwrap();
                Ok(next_deps)
            } else {
                let res_package =
                    filesystem::generate_lock_file(resolved, next_deps.clone()).unwrap();
                //use the values returned to update package.json
                filesystem::update_package_jason_dep(res_package, update).unwrap();
                Ok(next_deps)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
