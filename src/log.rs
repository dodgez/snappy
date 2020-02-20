use colored::*;
use std::io;

use crate::objects::Commit;
use crate::{branch, hash, repo};

pub fn log() -> Result<(), io::Error> {
    let repo = repo::import()?;

    let mut commit_hash = branch::get_latest_commit()?;
    while commit_hash != "0" {
        let hash_path = repo.snaps_dir.join(hash::get_hash_path(&commit_hash));
        let commit = Commit::from_file(&hash_path)?;
        println!(
            "Commit: {}\nParent: {}\nMessage: {}\nAuthor: {}\nTree: {}\n",
            commit.hash.green(),
            commit.parent,
            commit.message,
            commit.author,
            commit.tree
        );

        commit_hash = commit.parent;
    }

    Ok(())
}
