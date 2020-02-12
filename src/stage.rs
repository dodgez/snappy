use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn stage_file(path: &Path) {
  let snap_dir = Path::new(".snappy");
  let snaps_dir = snap_dir.join("snaps");

  if !path.is_relative() {
    panic!("fatal: only relative paths are supported");
  }

  let mut hasher = Sha256::default();
  let mut data = Vec::<u8>::new();
  File::open(path).unwrap().read_to_end(&mut data).unwrap();
  hasher.input(&data);
  let hash = format!("{:x}", hasher.result());
  
  let hash_dir = snaps_dir.join(&hash[0..2]);
  if !hash_dir.exists() {
    create_dir_all(&hash_dir).unwrap();
  }
  let file_path = hash_dir.join(&hash[2..]);
  let mut file = File::create(file_path).unwrap();
  file.write("file\0".as_bytes()).unwrap();
  file.write(&data).unwrap();

  let mut ancestors = path.ancestors();
  while let Some(_ancestor) = ancestors.next() {
    // update the above directories
  }
}

fn stage_dir(_path: &Path) {
  panic!("fatal: not implemented yet");
}

pub fn stage(path: &Path) {
  if !path.exists() {
    panic!("fatal: object does not exist {}", path.display());
  }

  if path.is_dir() {
    stage_dir(path);
  } else if path.is_file() {
    stage_file(path);
  }
}
