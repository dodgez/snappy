use std::fs::read_to_string;
use std::path::Path;

use crate::hash;
use crate::objects::Commit;

pub fn log() {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let mut commit_hash = read_to_string(head_file).unwrap();
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
