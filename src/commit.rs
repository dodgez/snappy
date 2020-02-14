use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write, File};
use std::io::prelude::*;
use std::path::Path;

struct HashObject {
    name: String,
    hash: String,
}

fn track_object(path: &Path) {
    let snap_dir = Path::new(".snappy");
    let tracked_file = snap_dir.join("tracked");

    if !tracked_file.exists() {
        File::create(&tracked_file).unwrap();
    }

    let contents = read_to_string(&tracked_file).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if path == Path::new(line) {
            return;
        }
    }

    write(tracked_file, contents + &format!("{}\n", path.display())).unwrap();
}

fn recurse_dir_commit(path: &Path) -> HashObject {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");

    let mut contents = read_dir(path).unwrap();
    let mut hashes = Vec::<HashObject>::new();
    while let Some(dir_entry) = contents.next() {
        let entry_path = dir_entry.unwrap().path();

        if entry_path.is_dir() {
            hashes.push(recurse_dir_commit(
                &path.join(entry_path.file_name().unwrap()),
            ));
        } else if entry_path.is_file() {
            hashes.push(HashObject {
                name: entry_path.file_name().unwrap().to_str().unwrap().to_owned(),
                hash: read_to_string(entry_path).unwrap(),
            });
        }
    }

    let mut bytes = Vec::<u8>::new();
    for i in 0..hashes.len() {
        let hash = &hashes[i];
        bytes.extend_from_slice(format!("{}:{}", hash.name, hash.hash).as_bytes());
        if i < hashes.len() - 1 {
            bytes.extend_from_slice("\n".as_bytes());
        }
    }

    let mut hasher = Sha256::default();
    hasher.input(&bytes);
    let hash = format!("{:x}", hasher.result());
    let hash_dir = snaps_dir.join(&hash[0..2]);
    if !hash_dir.exists() {
        create_dir_all(&hash_dir).unwrap();
    }
    let file_path = hash_dir.join(&hash[2..]);
    if !file_path.exists() {
        let mut file = File::create(file_path).unwrap();
        file.write("tree\0".as_bytes()).unwrap();
        file.write(&bytes).unwrap();
    }

    return HashObject {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        hash,
    };
}

pub fn commit(message: &str) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let temp_dir = snap_dir.join("commit-temp");
    let head_file = snap_dir.join("HEAD");
    let index_file = snap_dir.join("index");

    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    create_dir_all(&temp_dir).unwrap();

    let contents = read_to_string(&index_file).unwrap();
    let mut files = contents.lines();

    while let Some(file) = files.next() {
        let file_info = file.split(':').collect::<Vec<&str>>();
        let path = Path::new(file_info[0]);
        match path.parent() {
            Some(parent) => track_object(parent),
            None => (),
        }
        track_object(path);
        let hash = file_info[1];
        let temp_file_path = temp_dir.join(path);
        match temp_file_path.parent() {
            Some(parent) => create_dir_all(parent).unwrap(),
            None => (),
        }
        write(temp_file_path, hash.as_bytes()).unwrap();
    }

    let final_hash = recurse_dir_commit(&temp_dir);
    let mut bytes = Vec::<u8>::new();
    bytes.extend_from_slice(
        format!(
            "{}\n{}\n{}",
            read_to_string(&head_file).unwrap(),
            message,
            final_hash.hash
        )
        .as_bytes(),
    );
    let mut hasher = Sha256::default();
    hasher.input(&bytes);
    let hash = format!("{:x}", hasher.result());
    let hash_dir = snaps_dir.join(&hash[0..2]);
    if !hash_dir.exists() {
        create_dir_all(&hash_dir).unwrap();
    }
    let file_path = hash_dir.join(&hash[2..]);
    let mut file = File::create(file_path).unwrap();
    file.write("comm\0".as_bytes()).unwrap();
    file.write(&bytes).unwrap();

    remove_dir_all(temp_dir).unwrap();
    write(head_file, hash.as_bytes()).unwrap();

    println!("{}", hash);
}
