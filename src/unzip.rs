use flate2::read::GzDecoder; // Add this import for Gzip support
use indicatif::ProgressBar;
use std::fs;
// use std::io::copy;
use std::io::{copy, BufReader};
use std::path::{Path, PathBuf};
//import console create
use crate::console;
use tar::Archive;
// use ureq;
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};
pub fn extract_tarball_to_disk(url: &str, package_name: &str) {
    //create ureq agent
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    // URL of the tar file
    // let url = "https://example.com/path/to/your.tar.gz";

    // Destination folder
    // let dest_folder = "./node_tests/node_modules";
    let dest_folder = format!("./node_tests/node_modules/{}", package_name);

    // Create the destination folder if it doesn't exist
    if !Path::new(dest_folder.as_str()).exists() {
        std::fs::create_dir_all(&dest_folder).expect("Failed to create destination folder");
    }

    // Download the tar file using ureq
    // let bar = ProgressBar::new(1000).with_prefix("Downloading");
    let bar = ProgressBar::new(!0)
        .with_prefix("Downloading")
        .with_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{prefix:>12.bright.cyan} {spinner} {msg:.cyan}")
                .unwrap(),
        )
        .with_message("Done");
    let response = agent.get(url).call();
    // Create a temporary file to store the downloaded tar file
    /***
     *
     * Handle any issues encountered while downloading tar
     * be able to show progress information of the download
     *
     *
     */
    match response {
        Ok(response) => {
            let mut temp_file = fs::File::create("./node_tests/node_modules/temp.tar.gz")
                .expect("Failed to create temp file");
            //
            if let Some(length) = response
                .header("content-length")
                .and_then(|l| l.parse().ok())
            {
                bar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{prefix:>12.bright.cyan} [{bar:27}] {bytes:>9}/{total_bytes:9}  {bytes_per_sec}  ETA {eta:4} - {msg:.cyan}").unwrap()
                    .progress_chars("=> "));
                bar.set_length(length);
            } else {
                bar.println("Length unspecified, expect at least 250MiB");
                bar.set_style(indicatif::ProgressStyle::default_spinner().template(
                "{prefix:>12.bright.cyan} {spinner} {bytes:>9} {bytes_per_sec} - {msg:.cyan}",
            ).unwrap());
            }
            let mut res = bar.wrap_read(response.into_reader());
            // Copy the response body to the temporary file
            bar.finish_and_clear();
            copy(&mut res, &mut temp_file).expect("Failed to copy response body to file");

            // Open the downloaded tar file
            let tar_file = fs::File::open("./node_tests/node_modules/temp.tar.gz")
                .expect("Failed to open tar file");
            // Use Gzip decoder for decompression
            let tar_reader = BufReader::new(GzDecoder::new(tar_file));
            // Create a tar archive from the file
            let mut archive = Archive::new(tar_reader);
            // Extract the contents of the tar file to the custom project folder
            // ** we also remove the default /package from the tar returned by NPM**
            archive
                .entries()
                .expect("Failed to get tar entries")
                .for_each(|entry| {
                    let mut entry = entry.expect("Failed to get tar entry");

                    // Handle variations in the directory structure
                    let entry_path = entry.path().expect("Failed to get entry path");
                    let relative_path = entry_path
                        .strip_prefix("package/")
                        .unwrap_or_else(|_| &entry_path); // Use original path if strip_prefix fails

                    let dest_path = PathBuf::from(&dest_folder).join(relative_path);

                    // Ensure the parent directory exists
                    if let Some(parent_dir) = dest_path.parent() {
                        std::fs::create_dir_all(parent_dir)
                            .expect("Failed to create parent directory");
                    }

                    // Unpack the entry to the adjusted destination path
                    entry
                        .unpack(&dest_path)
                        .expect("Failed to unpack tar entry");
                });

            // Cleanup: Remove the temporary tar file
            std::fs::remove_file("./node_tests/node_modules/temp.tar.gz")
                .expect("Failed to remove temp file");
            let message = format!("{} has been successfully downloaded", package_name);
            console::show_success(message)
            // println!("Tar file has been successfully downloaded and unpacked.");
        }
        Err(Error::Status(_code, _response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
            eprint!("Error code from the server");
        }
        Err(_) => {
            /* some kind of io/transport error */
            eprintln!("Failed please check your connection")
        }
    }
}
