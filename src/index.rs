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
    let lines = lines.collect::<Vec<&str>>();
    let mut new_lines = Vec::<&str>::new();
    let updated_line = &TreeEntry {
        hash: hash.to_string(),
        name: path.display().to_string(),
    }
    .to_string();
    let mut found_line = false;

    for line in lines {
        if line.starts_with(&path.display().to_string()) {
            new_lines.push(updated_line);
            found_line = true;
        } else {
            new_lines.push(line);
        }
    }

    if !found_line {
        new_lines.push(updated_line);
    }

    fs::write(repo.index_file, new_lines.join("\n").as_bytes())?;

    Ok(())
}
