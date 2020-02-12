use std::fs::{create_dir_all, File, remove_dir_all};
use std::path::Path;

pub fn init(force: bool) {
  let snap_dir = Path::new(".snappy");
  let snaps_dir = snap_dir.join("snaps");
  let head_file = snap_dir.join("HEAD");

  if snap_dir.exists() {
    if force {
      remove_dir_all(snap_dir).unwrap();
    } else {
      panic!("fatal: found an existing snappy repo");
    }
  }

  create_dir_all(snap_dir).unwrap();
  create_dir_all(snaps_dir).unwrap();

  File::create(head_file).unwrap();
}