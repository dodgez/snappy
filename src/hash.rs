use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(data.as_bytes());
    return format!("{:x}", hasher.result());
}

pub fn get_hash_path(hash: &str) -> PathBuf {
    let hash_dir = Path::new(&hash[0..2]);
    hash_dir.join(&hash[2..])
}

pub fn create_hash_dir(hash: &str, base_path: &Path) -> Result<(), io::Error> {
    let hash_dir = base_path.join(&hash[0..2]);
    if !hash_dir.exists() {
        fs::create_dir_all(hash_dir)?;
    }

    Ok(())
}
