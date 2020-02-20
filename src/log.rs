use colored::*;
use std::io;
use std::path::Path;

use crate::branch::get_latest_commit;
use crate::hash;
use crate::objects::Commit;

pub fn log() -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let mut commit_hash = get_latest_commit()?;
    while commit_hash != "0" {
        let hash_path = snaps_dir.join(hash::get_hash_path(&commit_hash));
        let commit = Commit::from_file(&hash_path)?;
        println!(
            "Commit: {}\nParent: {}\nMessage: {}\nAuthor: {}\nTree: {}\n",
            commit.hash.green(), commit.parent, commit.message, commit.author, commit.tree
        );

        commit_hash = commit.parent;
    }

    Ok(())
}
