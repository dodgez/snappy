use glob::Pattern;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::Path;

use crate::hash::create_hash_dir;
use crate::index::update_index;
use crate::objects::File;
use crate::repo::import;

fn stage_file(path: &Path) -> Result<(), io::Error> {
    let repo = import()?;
    if !path.is_relative() {
        panic!("fatal: only relative paths are supported");
    }

    let file = File::new(read_to_string(path)?);
    create_hash_dir(&file.hash, &repo.snaps_dir)?;
    file.write_to_file(&repo.snaps_dir.join(file.get_hash_path()))?;

    update_index(path, &file.hash)?;

    Ok(())
}

fn stage_dir(path: &Path) -> Result<(), io::Error> {
    let mut iter = read_dir(path)?;
    while let Some(dir_entry) = iter.next() {
        let path = dir_entry?.path();
        if !path.display().to_string().contains(".snappy") {
            stage(&path)?;
        }
    }

    Ok(())
}

pub fn stage(path: &Path) -> Result<(), io::Error> {
    if !path.exists() {
        panic!("fatal: object does not exist {}", path.display());
    }

    let ignore_file = Path::new(".snapignore");
    if ignore_file.exists() {
        let contents = read_to_string(&ignore_file)?;
        let mut lines = contents.lines();
        while let Some(line) = lines.next() {
            let ignored = Pattern::new(&line).unwrap();
            if ignored.matches_path(&path) {
                return Ok(());
            }
        }
    }
    if path.is_dir() {
        stage_dir(path)?;
    } else if path.is_file() {
        stage_file(path)?;
    }

    Ok(())
}
