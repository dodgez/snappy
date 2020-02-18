use std::fs::read_to_string;
use std::io;
use std::path::Path;

use crate::branch::{get_latest_commit, update_branch};
use crate::hash::get_hash_path;
use crate::objects::Commit;

fn is_commit_parent(child: &str, parent: &str) -> Result<bool, io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let mut hash = child.to_string();

    while hash != "0" {
        let child_file = snaps_dir.join(get_hash_path(&hash));
        let child_commit = Commit::from_file(&child_file)?;

        if child_commit.hash == parent {
            return Ok(true);
        }

        hash = child_commit.parent;
    }

    Ok(false)
}

pub fn merge(object: &str) -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let branches_dir = snap_dir.join("branches");
    
    let mut commit = object.to_string();
    let branch_file = branches_dir.join(object);
    if branch_file.exists() {
        commit = read_to_string(branch_file)?;
    }

    if !is_commit_parent(&commit, &get_latest_commit()?)? {
        panic!("fatal: only fast forward merges are implemented");
    }

    update_branch(&commit)?;

    Ok(())
}
