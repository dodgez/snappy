use std::fs::{create_dir_all, read_to_string, remove_dir, remove_file, write};
use std::io;
use std::path::Path;

use crate::branch::update_head;
use crate::hash::get_hash_path;
use crate::index::update_index;
use crate::objects::{Commit, File, Tree};

fn populate_working_directory(hash: &str, partial_path: &Path) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let hash_file = snaps_dir.join(get_hash_path(&hash));
    if !hash_file.exists() {
        panic!("fatal: object {} does not exist", &hash);
    }

    let contents = read_to_string(hash_file)?;
    if contents.starts_with("file") {
        let file = File::from_string(&contents);
        update_index(&partial_path, &hash)?;
        write(partial_path, file.contents)?;
    } else if contents.starts_with("tree") {
        if partial_path != Path::new("") {
            create_dir_all(partial_path)?;
        }
        let tree = Tree::from_string(&contents);
        let partial_path = partial_path.to_path_buf();
        for child in tree.children {
            let path = partial_path.join(child.name);

            populate_working_directory(&child.hash, &path)?;
        }
    }

    Ok(())
}

pub fn checkout(commit_hash: &str) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let index_file = snap_dir.join("index");
    let tracked_file = snap_dir.join("tracked");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    if index_file.exists() {
        remove_file(index_file)?;
    }

    let mut commit = commit_hash.to_string();
    let branch_file = branches_dir.join(commit_hash);
    if branch_file.exists() {
        commit = read_to_string(branch_file)?;
    }
    let commit_file = snaps_dir.join(get_hash_path(&commit));
    let commit = Commit::from_file(&commit_file)?;
    let tree_hash = commit.tree;

    let tracked_contents = read_to_string(tracked_file)?;
    let mut tracked_objects = tracked_contents.lines();
    while let Some(object) = tracked_objects.next() {
        let path = Path::new(object);
        if path.exists() {
            if path.is_dir() {
                match remove_dir(path) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            } else if path.is_file() {
                remove_file(path)?;

                if let Some(parent) = path.parent() {
                    match remove_dir(parent) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }
        }
    }

    populate_working_directory(&tree_hash, Path::new(""))?;

    update_head(commit_hash)?;

    Ok(())
}
