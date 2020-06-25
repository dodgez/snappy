use std::{fs, io};

use crate::{hash, repo};

pub fn get_latest_commit() -> Result<String, io::Error> {
    let repo = repo::import()?;

    let branch = fs::read_to_string(repo.head_file)?;
    let branch_file = repo.branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = repo.snaps_dir.join(hash::get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        return Ok(branch);
    }

    Ok(fs::read_to_string(branch_file)?)
}

pub fn update_branch(commit: &str) -> Result<(), io::Error> {
    let repo = repo::import()?;

    let branch = fs::read_to_string(&repo.head_file)?;
    let branch_file = repo.branches_dir.join(&branch);
    if !branch_file.exists() {
        let commit_file = repo.snaps_dir.join(hash::get_hash_path(&branch));
        if !commit_file.exists() {
            panic!("fatal: branch {} does not exist", branch);
        }

        fs::write(repo.head_file, commit.as_bytes())?;
    }

    fs::write(branch_file, commit.as_bytes())?;

    Ok(())
}

pub fn update_head(commit: &str) -> Result<(), io::Error> {
    let repo = repo::import()?;

    fs::write(repo.head_file, commit.as_bytes())?;

    Ok(())
}

pub fn branch(branch: &str) -> Result<(), io::Error> {
    let repo = repo::import()?;

    let branch_file = repo.branches_dir.join(&branch);
    if branch_file.exists() {
        panic!("fatal: branch {} already exists", branch);
    }

    let commit = get_latest_commit()?;
    fs::write(branch_file, commit.as_bytes())?;
    fs::write(repo.head_file, branch.as_bytes())?;

    Ok(())
}
