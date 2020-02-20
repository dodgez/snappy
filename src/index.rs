use std::path::Path;
use std::{fs, io};

use crate::objects::TreeEntry;
use crate::repo;

pub fn update_index(path: &Path, hash: &str) -> Result<(), io::Error> {
    let path = if path.starts_with(".") {
        path.strip_prefix(".").unwrap()
    } else {
        path
    };
    let repo = repo::import()?;

    let contents = fs::read_to_string(&repo.index_file)?;
    let lines = contents.lines();
    let mut lines = lines.collect::<Vec<&str>>();
    let updated_line = &TreeEntry {
        hash: hash.to_string(),
        name: path.display().to_string(),
    }
    .to_string();
    let mut found_line = false;

    for i in 0..lines.len() {
        if lines[i].starts_with(&path.display().to_string()) {
            lines[i] = updated_line;
            found_line = true;
        }
    }

    if !found_line {
        lines.push(updated_line);
    }

    fs::write(repo.index_file, lines.join("\n").as_bytes())?;

    Ok(())
}
