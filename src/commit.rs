use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write};
use std::io;
use std::path::Path;

use crate::branch::{get_latest_commit, update_branch};
use crate::hash::create_hash_dir;
use crate::objects::{Commit, Tree, TreeEntry};
use crate::repo::import;

fn track_object(path: &Path) -> Result<(), io::Error> {
    let repo = import()?;

    let contents = read_to_string(&repo.tracked_file)?;
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if path == Path::new(line) {
            return Ok(());
        }
    }

    write(
        repo.tracked_file,
        contents + &format!("{}\n", path.display()),
    )?;

    Ok(())
}

fn recurse_dir_commit(path: &Path) -> Result<TreeEntry, io::Error> {
    let repo = import()?;

    let mut contents = read_dir(path)?;
    let mut children = Vec::<TreeEntry>::new();
    while let Some(dir_entry) = contents.next() {
        let entry_path = dir_entry?.path();

        if entry_path.is_dir() {
            children.push(recurse_dir_commit(
                &path.join(entry_path.file_name().unwrap()),
            )?);
        } else if entry_path.is_file() {
            children.push(TreeEntry {
                hash: read_to_string(&entry_path)?,
                name: entry_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            });
        }
    }

    let tree = Tree::new(children);
    create_hash_dir(&tree.hash, &repo.snaps_dir)?;
    tree.write_to_file(&repo.snaps_dir.join(tree.get_hash_path()))?;

    Ok(TreeEntry {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        hash: tree.hash,
    })
}

pub fn commit(message: &str, author: &str) -> Result<String, io::Error> {
    let repo = import()?;
    let temp_dir = repo.snap_dir.join("commit-temp");

    create_dir_all(&temp_dir)?;

    let contents = read_to_string(&repo.index_file)?;
    let mut files = contents.lines();

    while let Some(entry) = files.next() {
        let file_info = TreeEntry::from_string(entry);
        let path = Path::new(&file_info.name);
        match path.parent() {
            Some(parent) => track_object(parent)?,
            None => (),
        }
        track_object(path)?;
        let file_path = temp_dir.join(path);
        match file_path.parent() {
            Some(parent) => create_dir_all(parent)?,
            None => (),
        }
        write(file_path, file_info.hash.as_bytes())?
    }

    let tree = recurse_dir_commit(&temp_dir)?;
    let commit = Commit::new(
        get_latest_commit()?,
        message.to_string(),
        author.to_string(),
        tree.hash,
    );
    create_hash_dir(&commit.hash, &repo.snaps_dir)?;
    commit.write_to_file(&repo.snaps_dir.join(commit.get_hash_path()))?;

    remove_dir_all(temp_dir)?;
    update_branch(&commit.hash)?;

    return Ok(commit.hash);
}
