use sha2::{Digest, Sha256};

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(data.as_bytes());
    return format!("{:x}", hasher.result());
}
