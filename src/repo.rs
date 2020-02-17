use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

pub fn init(force: bool) {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let branches_dir = snap_dir.join("branches");
    let master_branch_file = branches_dir.join("master");
    let head_file = snap_dir.join("HEAD");
    let index_file = snap_dir.join("index");
    let tracked_file = snap_dir.join("tracked");

    if snap_dir.exists() {
        if force {
            remove_dir_all(snap_dir).unwrap();
        } else {
            panic!("fatal: found an existing snappy repository");
        }
    }

    create_dir_all(snap_dir).unwrap();
    create_dir_all(snaps_dir).unwrap();
    create_dir_all(branches_dir).unwrap();

    File::create(master_branch_file)
        .unwrap()
        .write("0".as_bytes())
        .unwrap();
    File::create(head_file)
        .unwrap()
        .write("master".as_bytes())
        .unwrap();
    File::create(index_file).unwrap();
    File::create(tracked_file).unwrap();
}
