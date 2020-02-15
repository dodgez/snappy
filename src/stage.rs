use glob::Pattern;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

use crate::objects::File;
use crate::{hash, index};

fn stage_file(path: &Path) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }
    if !path.is_relative() {
        panic!("fatal: only relative paths are supported");
    }

    let file = File::new(read_to_string(path).unwrap());
    hash::create_hash_dir(&file.hash, &snaps_dir);
    file.write_to_file(&file.get_hash_path());

    index::update_index(path, &file.hash);
}

fn stage_dir(path: &Path) {
    let mut iter = read_dir(path).unwrap();
    while let Some(dir_entry) = iter.next() {
        let path = dir_entry.unwrap().path();
        if !path.display().to_string().contains(".snappy") {
            stage(&path);
        }
    }
}

pub fn stage(path: &Path) {
    if !path.exists() {
        panic!("fatal: object does not exist {}", path.display());
    }

    let ignore_file = Path::new(".snapignore");
    if ignore_file.exists() {
        let contents = read_to_string(&ignore_file).unwrap();
        let mut lines = contents.lines();
        while let Some(line) = lines.next() {
            let ignored = Pattern::new(&line).unwrap();
            if ignored.matches_path(&path) {
                return;
            }
        }
    }
    if path.is_dir() {
        stage_dir(path);
    } else if path.is_file() {
        stage_file(path);
    }
}
