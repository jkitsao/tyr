// use clap::builder::Str;
// use clap::Error;
use ctrlc;
mod cli;
mod filesystem;
mod http;
mod init;
mod install;
mod semvar;
mod unzip;
mod utils;
// use serde_json::json;
use std::collections::{BTreeMap, HashMap};
mod banner;
mod reconsole;
// mod dialogue;

use std::process::ExitCode;

use serde_json::Value;
// use std::collections::HashMap;
// use std::fmt::Debug;
// use std::env;
use console::{style, Emoji, Term};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
// use std::fs;
// use std::io::BufReader;
// use std::path::Path;
use std::time::Instant;
// use ureq::Error;

// use std::ops::ControlFlow;
// use std::path;
//for printing to the screen
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
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
        // return ControlFlow::Break();
        //exit with code
    })
    .expect("Error setting Ctrl-C handler");
    //
    let started = Instant::now();
    cli::initialize_command_arguments();
    println!(
        "{} {} {}",
        SPARKLE,
        style("Done in").yellow().bold().bright(),
        style(HumanDuration(started.elapsed()))
            .yellow()
            .bold()
            .bright()
    );
    ExitCode::SUCCESS
}
//resolve dependency/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided, but I'll start
pub fn resolve_package_from_registry(dep: String, update: bool) {
    // get the package name and version from user arg
    let (name, version) = semvar::split_package_version(&dep);
    //
    // dbg!("installing: {}", version.clone());
    //call installer function
    let next_deps = package_installer(name.clone(), version, update.clone());
    //iterate over next deps
    if let Ok(dependencies) = next_deps {
        //play with progress bar for deps
        let pb = ProgressBar::new(dependencies.len() as u64);
        pb.set_style(
            ProgressStyle::with_template(
                // note that bar size is fixed unlike cargo which is dynamic
                // and also the truncation in cargo uses trailers (`...`)
                if Term::stdout().size().1 > 80 {
                    "{prefix:>12.green}  {pos}/{len} {wide_msg}"
                } else {
                    "{prefix:>12.green}  {pos}/{len}"
                },
            )
            .unwrap()
            .progress_chars("** "),
        );
        for (key, value) in dependencies.iter() {
            let result = format!("{}@{}", key, value);
            let f = format!("[+] {}", result.clone());
            pb.inc(1);
            pb.set_prefix(f);
            // Don't update package.json
            // println!("the result is {}", result);
            resolve_package_from_registry(result, false);
        }
        pb.finish();
    } else if let Err(err) = next_deps {
        // eprintln!(" {:?}", err);
        println!("{} {}", PAPER, style(err).bright().yellow());
    }
}

// install package to disk
//installer function that resolves remote packages and arranges to disk
// static  mut is_update=true;

fn package_installer(
    name: String,
    version: String,
    update: bool,
) -> Result<BTreeMap<String, Value>, String> {
    // TODO: have it return the next dep to be resolved
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
                        pckg_data = data.to_owned();
                    }
                    Err(err) => {
                        eprint!("{err}");
                        pckg_data = resolved.get("dist").unwrap().clone();
                    }
                }
                // dbg!(&versions);
            } else {
                //update pckg_data to dist returned
                pckg_data = resolved.get("dist").unwrap().clone();
            }

            // dbg!(resolved.clone());
            // let name = pckg_data.get("name").unwrap();
            let _pckg_map: HashMap<String, Value> =
                serde_json::from_value(pckg_data.clone()).unwrap();
            // println!("resolved: {:?}", pckg_data);
            let next_dependencies = match resolved.contains_key("dependencies") {
                true => {
                    let res: BTreeMap<String, Value> =
                        serde_json::from_value(resolved.get("dependencies").unwrap().to_owned())
                            .unwrap();
                    Ok(res)
                }
                false => {
                    // let version = resolved.get("version").unwrap();
                    let msg = format!("Resolving dependencies for: {}", name.clone());
                    Err(msg)
                }
            };
            // dbg!(next_depss.unwrap().clone());
            let dist = pckg_data.clone();
            let tarball = dist.get("tarball").unwrap();
            let name = resolved.get("name").unwrap();
            // let _integrity = dist.get("integrity").unwrap();
            //Download and extract to file
            let _deps =
                unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
            match next_dependencies {
                Ok(dep) => {
                    let _pckg_map: HashMap<String, Value> =
                        serde_json::from_value(pckg_data.clone()).unwrap();
                    let res_package =
                        filesystem::generate_lock_file(resolved, dep.clone()).unwrap();
                    //use the values returned to update package.json
                    filesystem::update_package_jason_dep(res_package, update).unwrap();
                    Ok(dep)
                }
                Err(value) => {
                    // eprintln!("{}",value);
                    Err(value.to_string())
                }
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
