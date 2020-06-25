extern crate diff as file_diff;

use colored::*;
use file_diff::Result;
use std::path::Path;
use std::{fs, io};

use crate::objects::{File, TreeEntry};
use crate::{hash, repo};

fn print_diff(last: &str, current: &str) {
    for diff in file_diff::lines(last, current) {
        match diff {
            Result::Left(l) => println!("-{}", l.red()),
            Result::Both(l, _) => println!(" {}", l),
            Result::Right(r) => println!("+{}", r.green()),
        }
    }
}

pub fn diff(file: &str) -> std::result::Result<(), io::Error> {
    let path = Path::new(file);
    if !path.exists() {
        panic!("fatal: object does not exist {}", path.display());
    }

    let repo = repo::import()?;

    let current_contents = fs::read_to_string(&path)?;

    let contents = fs::read_to_string(repo.index_file)?;
    let files = contents.lines();

    for entry in files {
        if entry.starts_with(file) {
            let file_info = TreeEntry::from_string(entry);
            let file_path = hash::get_hash_path(&file_info.hash);
            let file_contents = File::from_file(&repo.snaps_dir.join(file_path))?;

            print_diff(&file_contents.contents, &current_contents);

            return Ok(());
        }
    }

    print_diff("", &current_contents);
    Ok(())
}
