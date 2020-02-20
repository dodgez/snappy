use std::{fs, io};

use crate::{branch, hash, repo};
use crate::objects::Commit;

fn is_commit_parent(child: &str, parent: &str) -> Result<bool, io::Error> {
    let repo = repo::import()?;
    let mut hash = child.to_string();

    while hash != "0" {
        let child_file = repo.snaps_dir.join(hash::get_hash_path(&hash));
        let child_commit = Commit::from_file(&child_file)?;

        if child_commit.hash == parent {
            return Ok(true);
        }

        hash = child_commit.parent;
    }

    Ok(false)
}

pub fn merge(object: &str) -> Result<(), io::Error> {
    let repo = repo::import()?;

    let mut commit = object.to_string();
    let branch_file = repo.branches_dir.join(object);
    if branch_file.exists() {
        commit = fs::read_to_string(branch_file)?;
    }

    if !is_commit_parent(&commit, &branch::get_latest_commit()?)? {
        panic!("fatal: only fast forward merges are implemented");
    }

    branch::update_branch(&commit)?;

    Ok(())
}
