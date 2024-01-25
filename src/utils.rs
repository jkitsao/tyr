// utility functions
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
//helper function that walks  a directory
// one possible implementation of walking a directory only visiting files
// use std::{fs, io};

pub fn visit_dir(path_name: String) -> io::Result<()> {
    let mut entries = fs::read_dir(path_name)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    println!("entries {:?}", entries);

    // The entries have now been sorted by their path.

    Ok(())
}
