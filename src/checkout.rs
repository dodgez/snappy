use std::fs::{create_dir_all, read_to_string, remove_dir_all, remove_file, write};
use std::path::Path;

fn populate_working_directory(hash: &str, partial_path: &Path) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let hash_dir = snaps_dir.join(&hash[0..2]);
    let hash_file = hash_dir.join(&hash[2..]);

    if !hash_file.exists() {
        panic!("fatal: object {} does not exist", &hash);
    }

    let mut object_contents = read_to_string(hash_file).unwrap();
    if object_contents.starts_with("file\0") {
        println!("Writing to file {}", partial_path.display());
        write(partial_path, &object_contents[5..]).unwrap();
    } else if object_contents.starts_with("tree\0") {
        if partial_path != Path::new("") {
            println!("Creating directory {}", partial_path.display());
            create_dir_all(partial_path).unwrap();
        }
        object_contents = object_contents[5..].to_string();
        let mut lines = object_contents.lines();
        while let Some(line) = lines.next() {
            let mut parts = line.split(':');
            let name = parts.next().unwrap();
            let hash = parts.next().unwrap();

            println!("Going into {} with hash {}", name, hash);
            let new_path = partial_path.to_path_buf().join(name);
            populate_working_directory(hash, &new_path);
        }
    }
}

pub fn checkout(commit_hash: &str) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let head_file = snap_dir.join("HEAD");
    let tracked_file = snap_dir.join("tracked");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let commit_hash_dir = snaps_dir.join(&commit_hash[0..2]);
    let commit_file = commit_hash_dir.join(&commit_hash[2..]);
    let commit_contents = read_to_string(commit_file).unwrap();
    let mut commit_lines = commit_contents.lines();
    let _parent = commit_lines.next().unwrap();
    let _message = commit_lines.next().unwrap();
    let tree_hash = commit_lines.next().unwrap();

    println!("Commit tree {}", tree_hash);

    let tracked_contents = read_to_string(tracked_file).unwrap();
    let mut tracked_objects = tracked_contents.lines();
    while let Some(object) = tracked_objects.next() {
        let path = Path::new(object);
        if path.exists() {
            if path.is_dir() {
                println!("Deleting directory {}", path.display());
                remove_dir_all(path).unwrap();
            } else if path.is_file() {
                println!("Deleting file {}", path.display());
                remove_file(path).unwrap();
            }
        }
    }

    populate_working_directory(tree_hash, Path::new(""));

    write(head_file, commit_hash.as_bytes()).unwrap();
}