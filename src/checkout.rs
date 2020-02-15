use std::fs::{create_dir_all, read_to_string, remove_dir_all, remove_file, write};
use std::path::Path;

use crate::hash;
use crate::objects::{Commit, File, Tree};

fn populate_working_directory(hash: &str, partial_path: &Path) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let hash_file = snaps_dir.join(hash::get_hash_dir(&hash));
    if !hash_file.exists() {
        panic!("fatal: object {} does not exist", &hash);
    }

    let contents = read_to_string(hash_file).unwrap();
    if contents.starts_with("file") {
        let file = File::from_string(&contents);
        write(partial_path, file.contents).unwrap();
    } else if contents.starts_with("tree") {
        if partial_path != Path::new("") {
            println!("Creating directory {}", partial_path.display());
            create_dir_all(partial_path).unwrap();
        }
        let tree = Tree::from_string(&contents);
        let partial_path = partial_path.to_path_buf();
        for child in tree.children {
            let path = partial_path.join(child.name);

            populate_working_directory(hash, &path);
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

    let commit_file = snaps_dir.join(hash::get_hash_dir(&commit_hash));
    let commit = Commit::from_file(&commit_file);
    let tree_hash = commit.tree;

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

    populate_working_directory(&tree_hash, Path::new(""));

    write(head_file, commit_hash.as_bytes()).unwrap();
}
