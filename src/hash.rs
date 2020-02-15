use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(data.as_bytes());
    return format!("{:x}", hasher.result());
}

pub fn get_hash_dir(hash: &str) -> PathBuf {
    let hash_dir = Path::new(&hash[0..2]);
    return hash_dir.join(&hash[2..]);
}
