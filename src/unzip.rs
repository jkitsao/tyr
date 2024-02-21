use crate::utils;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io::{copy, BufReader};
use std::path::{Path, PathBuf};
use std::{thread, time::Duration};
use tar::Archive;
use ureq::Error::Status;
use ureq::{Agent, AgentBuilder};

pub fn extract_tarball_to_disk(
    url: &str,
    package_name: &str,
) -> Result<BTreeMap<String, Value>, String> {
    //create ureq agent
    let agent: Agent = AgentBuilder::new().build();
    let dest_folder = format!("./node_modules/{}", package_name);
    // Create the destination folder if it doesn't exist
    if !Path::new(dest_folder.as_str()).exists() {
        fs::create_dir_all(&dest_folder).expect("Failed to create destination folder");
    }
    // Download the tar file using ureq
    let bar = ProgressBar::new(!0).with_prefix("Downloading").with_style(
        ProgressStyle::default_spinner()
            .template("{prefix:>12.bright.cyan} {spinner} {msg:.cyan}")
            .unwrap(),
    );

    let response = agent.get(url).call();
    /*
     *
     * Handle any issues encountered while downloading tar
     * be able to show progress information of the download
     *
     *
     */
    match response {
        Ok(response) => {
            // Create a temporary file to store the downloaded tar file
            let mut temp_file =
                fs::File::create("./node_modules/temp.tar.gz").expect("Failed to create temp file");
            //show download progress for tar file
            if let Some(length) = response
                .header("content-length")
                .and_then(|l| l.parse().ok())
            {
                bar.set_style(
                ProgressStyle::default_bar()
                    .template("{prefix:>12.bright.cyan} [{bar:27}] {bytes:>9}/{total_bytes:9}  {bytes_per_sec}  ETA {eta:4}").unwrap()
                    .progress_chars("=> "));
                bar.set_length(length);
            } else {
                bar.println("Length unspecified, expect at least 250MiB");
                bar.set_style(
                    ProgressStyle::default_spinner()
                        .template("{prefix:>12.bright.cyan} {spinner} {bytes:>9} {bytes_per_sec}")
                        .unwrap(),
                );
            }
            let mut res = bar.wrap_read(response.into_reader());
            // Copy the response body to the temporary file
            copy(&mut res, &mut temp_file).expect("Failed to copy response body to file");
            bar.finish_and_clear();
            // Open the downloaded tar file
            let tar_file =
                fs::File::open("./node_modules/temp.tar.gz").expect("Failed to open tar file");
            // Use Gzip decoder for decompression
            let tar_reader = BufReader::new(GzDecoder::new(tar_file));
            // Create a tar archive from the file
            let mut archive = Archive::new(tar_reader);
            //show progress update on this
            let ext_bar = ProgressBar::new_spinner();
            archive
                .entries()
                .expect("Failed to get tar entries")
                .for_each(|entry| {
                    let mut entry = entry.expect("Failed to get tar entry");

                    // Handle variations in the directory structure
                    let entry_path = entry.path().expect("Failed to get entry path");
                    let relative_path = entry_path
                        // ** we also need to remove the default /package from the tar returned by NPM**
                        .strip_prefix("package/")
                        .unwrap_or_else(|_| &entry_path); // Use original path if strip_prefix fails

                    let dest_path = PathBuf::from(&dest_folder).join(relative_path);
                    // Ensure the parent directory exists
                    if let Some(parent_dir) = dest_path.parent() {
                        fs::create_dir_all(parent_dir).expect("Failed to create parent directory");
                    }
                    let size = entry.header().entry_size().unwrap();
                    ext_bar.enable_steady_tick(Duration::from_millis(size));
                    ext_bar.set_style(
                        ProgressStyle::with_template("{spinner:.yellow} {msg}")
                            .unwrap()
                            // For more spinners check out the cli-spinners project:
                            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
                    );
                    ext_bar.set_message("Unpacking");
                    // Unpack the entry to the adjusted destination path
                    entry
                        .unpack(&dest_path)
                        .expect("Failed to unpack tar entry");
                });
            ext_bar.finish_and_clear();
            // Cleanup: Remove the temporary tar file
            fs::remove_file("./node_modules/temp.tar.gz").expect("Failed to remove temp file");
            let mut pckg_dest_folder = format!("./node_modules/{}/package.json", package_name);
            //check if there's an extra path inside first
            if !Path::new(pckg_dest_folder.as_str()).exists() {
                // expect("Failed to create destination folder");
                let new_path = utils::visit_dir(dest_folder.clone()).unwrap();
                let read_path = format!("{}/package.json", new_path);
                // handle the headache
                pckg_dest_folder = read_path
            }
            //read the dependencies and update lock file
            let file = fs::File::open(pckg_dest_folder.clone()).unwrap();
            let reader = BufReader::new(file);
            // Read the JSON contents of the file and assign to Hashmap.
            let mut json_file_data: BTreeMap<String, Value> =
                serde_json::from_reader(reader).unwrap();
            //if dep is available
            match json_file_data.contains_key("dependencies") {
                true => {
                    //update the dep object with installed package metadata
                    let current_dep: Value =
                        json_file_data.get_mut("dependencies").unwrap().clone();
                    let res: BTreeMap<String, Value> = serde_json::from_value(current_dep).unwrap();
                    Ok(res)
                }
                false => {
                    let message = "Dep object not found after unzip";
                    // println!("{}",message);
                    Err(message.to_string())
                    // probably the first package
                }
            }
        }
        Err(Status(503, r)) | Err(Status(429, r)) => {
            for _ in 1..4 {
                let retry: Option<u64> = r.header("retry-after").and_then(|h| h.parse().ok());
                let retry = retry.unwrap_or(5);
                eprintln!("{} for {}, retry in {}", r.status(), r.get_url(), retry);
                thread::sleep(Duration::from_secs(retry));
            }

            let message = "Failed please check your connection";
            Err(message.to_string())
        }
        Err(_) => {
            /* some kind of io/transport error */
            // eprintln!("Failed please check your connection");
            let message = "Failed please check your connection";
            // println!("{}",message);
            Err(message.to_string())
        }
    }
}
