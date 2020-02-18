use std::fs::{read_to_string, write};
use std::io;
use std::path::Path;

use crate::hash::get_hash_path;

pub fn get_latest_commit() -> Result<String, io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch = read_to_string(head_file)?;
    let branch_file = branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = snaps_dir.join(get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        return Ok(branch);
    }

    return Ok(read_to_string(branch_file)?);
}

pub fn update_branch(commit: &str) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch = read_to_string(&head_file)?;
    let branch_file = branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = snaps_dir.join(get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        write(head_file, commit.as_bytes())?;
    }

    write(branch_file, commit.as_bytes())?;

    Ok(())
}

pub fn update_head(commit: &str) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    write(head_file, commit.as_bytes())?;

    Ok(())
}

pub fn branch(branch: &str) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let branches_dir = snap_dir.join("branches");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let branch_file = branches_dir.join(&branch);
    if branch_file.exists() {
        panic!("fatal: branch {} already exists", branch);
    }

    let commit = get_latest_commit()?;
    write(branch_file, commit.as_bytes())?;
    write(head_file, branch.as_bytes())?;

    Ok(())
}
