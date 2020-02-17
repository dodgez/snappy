use std::fs::{read_to_string, write};
use std::path::Path;

use crate::hash::get_hash_path;

pub fn get_latest_commit() -> String {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("head_file");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch = read_to_string(head_file).unwrap();
    let branch_file = branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = snaps_dir.join(get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        return branch;
    }

    return read_to_string(branch_file).unwrap();
}

pub fn update_branch(commit: &str) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("head_file");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch = read_to_string(&head_file).unwrap();
    let branch_file = branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = snaps_dir.join(get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        write(head_file, commit.as_bytes()).unwrap();
    }

    write(branch_file, commit.as_bytes()).unwrap();
}

pub fn update_head(commit: &str) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snap");
    let head_file = snap_dir.join("head_file");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let commit_file = snaps_dir.join(get_hash_path(&commit));
    if !commit_file.exists() {
        panic!("fatal: commit {} does not exist", commit);
    }

    write(head_file, commit.as_bytes()).unwrap();
}

pub fn branch(branch: &str) {
    let snap_dir = Path::new(".snappy");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("head_file");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch_file = branches_dir.join(&branch);
    if branch_file.exists() {
        panic!("fatal: branch {} already exists", branch);
    }

    let commit = get_latest_commit();
    write(branch_file, commit.as_bytes()).unwrap();
    write(head_file, branch.as_bytes()).unwrap();
}
