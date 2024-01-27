// utility functions
use std::fs;
use std::io;
use std::path::PathBuf;
// use std::path::Path;
//helper function that walks  a directory
// one possible implementation of walking a directory only visiting files
// use std::{fs, io};

pub fn visit_dir(path_name: String) -> io::Result<String> {
    let mut entries = fs::read_dir(path_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    let paths: Vec<PathBuf> = entries.to_vec();
    // println!("entries {:?}", entries);
    let f = paths[0].to_str().unwrap();
    let file = String::from(f);
    // The entries have now been sorted by their path.
    Ok(file)
}
