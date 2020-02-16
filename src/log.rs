use std::path::Path;

use crate::branch::get_latest_commit;
use crate::hash;
use crate::objects::Commit;

pub fn log() {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let mut commit_hash = get_latest_commit();
    while commit_hash != "0" {
        let hash_path = snaps_dir.join(hash::get_hash_path(&commit_hash));
        let commit = Commit::from_file(&hash_path);
        println!(
            "Commit: {}\nParent: {}\nMessage: {}\nTree: {}\n",
            commit.hash, commit.parent, commit.message, commit.tree
        );

        commit_hash = commit.parent;
    }
}
