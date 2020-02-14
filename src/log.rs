use std::fs::read_to_string;
use std::path::Path;

pub fn log() {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let head_file = snap_dir.join("HEAD");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    let mut commit_hash = read_to_string(head_file).unwrap();
    while commit_hash != "0" {
        let hash_dir = snaps_dir.join(&commit_hash[0..2]);
        let hash_file = hash_dir.join(&commit_hash[2..]);
        let commit_info = read_to_string(hash_file).unwrap();
        let mut commit_info = commit_info[5..].lines();
        let parent = commit_info.next().unwrap();
        let message = commit_info.next().unwrap();
        let tree_hash = commit_info.next().unwrap();

        println!("Commit {}\nParent {}\nMessage {}\nTree {}\n", commit_hash, parent, message, tree_hash);

        commit_hash = parent.to_string();
    }
}
