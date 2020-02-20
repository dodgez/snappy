extern crate diff as file_diff;

use colored::*;
use file_diff::{lines, Result};
use std::fs::read_to_string;
use std::io;

use crate::hash::get_hash_path;
use crate::objects::{File, TreeEntry};
use crate::repo::import;

fn print_diff(last: &str, current: &str) {
    for diff in lines(last, current) {
        match diff {
            Result::Left(l) => println!("-{}", l.red()),
            Result::Both(l, _) => println!(" {}", l),
            Result::Right(r) => println!("+{}", r.green()),
        }
    }
}

pub fn diff(file: &str) -> std::result::Result<(), io::Error> {
    let repo = import()?;

    let current_contents = read_to_string(file)?;

    let contents = read_to_string(repo.index_file)?;
    let mut files = contents.lines();

    while let Some(entry) = files.next() {
        if entry.starts_with(file) {
            let file_info = TreeEntry::from_string(entry);
            let file_path = get_hash_path(&file_info.hash);
            let file_contents = File::from_file(&repo.snaps_dir.join(file_path))?;

            print_diff(&file_contents.contents, &current_contents);

            return Ok(());
        }
    }

    print_diff("", &current_contents);
    Ok(())
}
