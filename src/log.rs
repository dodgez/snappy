use colored::*;
use std::io;

use crate::branch::get_latest_commit;
use crate::hash;
use crate::objects::Commit;
use crate::repo::import;

pub fn log() -> Result<(), io::Error> {
    let repo = import()?;

    let mut commit_hash = get_latest_commit()?;
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
