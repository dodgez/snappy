use glob::Pattern;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, read_dir, read_to_string, File};
use std::io::prelude::*;
use std::path::Path;

use crate::index;

fn stage_file(path: &Path) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");

    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }
    if !path.is_relative() {
        panic!("fatal: only relative paths are supported");
    }

    let mut hasher = Sha256::default();
    let mut data = Vec::<u8>::new();
    File::open(path).unwrap().read_to_end(&mut data).unwrap();
    hasher.input(&data);
    let hash = format!("{:x}", hasher.result());

    let hash_dir = snaps_dir.join(&hash[0..2]);
    if !hash_dir.exists() {
        create_dir_all(&hash_dir).unwrap();
    }
    let file_path = hash_dir.join(&hash[2..]);
    let mut file = File::create(file_path).unwrap();
    file.write("file\0".as_bytes()).unwrap();
    file.write(&data).unwrap();

    index::update_index(path, &hash);
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
