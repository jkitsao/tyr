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
use std::collections::BTreeMap;
mod reconsole;
use std::process::ExitCode;

use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
// use std::env;
use console::{style, Emoji};
use indicatif::HumanDuration;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use std::ops::ControlFlow;
// use std::path;
//for printing to the screen
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static _PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
//update structure
struct JsonFile {
    should_update:bool
}
fn main() ->ExitCode  {
    // graceful shutdown
    ctrlc::set_handler(move || {
        println!("{}",style("Received interrupt signal (Ctrl+C). Gracefully shutting down...
    ").bold().yellow().bright());
        // return ControlFlow::Break();
        //exit with code
    })
        .expect("Error setting Ctrl-C handler");
    //
    let started = Instant::now();
    cli::initialize_command_arguments();
    println!("{} {} {}", SPARKLE,style("Done in").yellow().bold().bright(),style(HumanDuration(started.elapsed())).yellow().bold().bright());
    ExitCode::SUCCESS
}
//resolve dependency/impl add command
//first step is to download the passed dep(is-even) from npm to a node_modules folder
//then add the package version to the prev created json! metadata appending `dependency` with `version`
// proceed to generate a lock file with
//also parse semver version if provided, but I'll start
pub fn resolve_package_from_registry(dep: String,update:bool) {
    // let mut update=true;
    //get required values from the string
    let (name, version) = semvar::split_package_version(&dep);
    // println!("package is {}, and the version is {}", name, version);
    //     let mut update =  JsonFile {
    //     should_update:true
    // };
    match version.as_ref() {
        "latest" => {
            println!(
                "{} {}Resolving packages...",
                style("[1/4]").bold().dim(),
                LOOKING_GLASS
            );

            let next_deps = package_installer(name.clone(), version,update.clone());
            if let Some((key, value)) = next_deps.iter().next() {
                let result = format!("{}@{}", key, value);
                //don't update package.json
                // println!("the result is {}", result);//
                resolve_package_from_registry(result,false);
            } else {
                println!("BTreeMap is empty");
            }
        }
        //semvar string has been passed
        _ => {
            let next_deps = package_installer(name.clone(), version,update.clone());
            match next_deps.iter().next() {
                Some((key, value)) => {
                    let result = format!("{}@{}", key, value);
                    //don't update package.json
                    // println!(" the result is {}", result);
                    resolve_package_from_registry(result,false);
                }
                None => {
                    println!("BTreeMap is empty (No more Dependencies)");
                }
            }

        }
    }
}

// install package to disk
//installer function that resolves remote packages and arranges to disk
// static  mut is_update=true;

fn package_installer(name: String, version: String,  update:bool) -> BTreeMap<String, Value> {
    // TODO: have it return the next dep to be resolved
    // call download package from registry function with
    let res = http::resolve_remote_package(name, version);
    //Match for response resolved or error
    match res {
        Ok(resolved)=>{
            // Now 'resolved' contains the parsed JSON data as a HashMap
            println!("resolved: {:?}",resolved);
            let dist = resolved.get("dist").unwrap();
            let version = resolved.get("version").unwrap();
            let tarball = dist.get("tarball").unwrap();
            let _integrity = dist.get("integrity").unwrap();
            let name = resolved.get("name").unwrap();
            // println!("the tar is {:?} and version is {:?}", tarball, version);
            // println!("proceeding to install {}  version {}", name, version);
            if update {
                //Actual package req
                println!(
                    "{} {}Fetching {} version {}",
                    style("[3/4]").bold().dim(),
                    CLIP,
                    style(&name).bold().cyan().bright(),
                    style(version).bold().cyan().bright()
                );
            }
            else {
                //peer packages
                // const CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
                // println!(
                //     "{} {}",
                //     CLIP,
                //     style("peer dependency").italic().bright().green()
                println!(
                    "{} {}Resolving Dependency {} version {}",
                    style("[2/4]").bold().dim(),
                    TRUCK,
                    style(&name).cyan(),
                    style(version).cyan()
                );
            }
            //Download and extract to file
            let deps = unzip::extract_tarball_to_disk(tarball.as_str().unwrap(), name.as_str().unwrap());
            //create/update a lock file
            let res_package = filesystem::generate_lock_file(resolved.clone(),deps.clone()).unwrap();
            //use the values returned to update package.json
            //also updates/creates dep in package.json
            //create or update dependencies
            // println!("Next deps is ..... are we updating  {:?} {:?}",deps,is_update);
            filesystem::update_package_jason_dep(res_package, update).unwrap();
            // is_update=false;
            // is_update=false;
            // println!("Next deps is ..... are we updating  {:?} {:?}",deps,is_update);
            deps
        }
        Err(err) =>{
            println!("{:?}",err);
            err
        }
    }

}
