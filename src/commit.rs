use std::path::Path;
use std::{fs, io};

use crate::objects::{Commit, Tree, TreeEntry};
use crate::{branch, hash, repo};

fn track_object(path: &Path) -> Result<(), io::Error> {
    let repo = repo::import()?;

    let contents = fs::read_to_string(&repo.tracked_file)?;
    let lines = contents.lines();
    for line in lines {
        if path == Path::new(line) {
            return Ok(());
        }
    }

    fs::write(
        repo.tracked_file,
        contents + &format!("{}\n", path.display()),
    )?;

    Ok(())
}

fn recurse_dir_commit(path: &Path) -> Result<TreeEntry, io::Error> {
    let repo = repo::import()?;

    let contents = fs::read_dir(path)?;
    let mut children = Vec::<TreeEntry>::new();
    for dir_entry in contents {
        let entry_path = dir_entry?.path();

        if entry_path.is_dir() {
            children.push(recurse_dir_commit(
                &path.join(entry_path.file_name().unwrap()),
            )?);
        } else if entry_path.is_file() {
            children.push(TreeEntry {
                hash: fs::read_to_string(&entry_path)?,
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
    hash::create_hash_dir(&tree.hash, &repo.snaps_dir)?;
    tree.write_to_file(&repo.snaps_dir.join(tree.get_hash_path()))?;

    Ok(TreeEntry {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        hash: tree.hash,
    })
}

pub fn commit(message: &str, author: &str) -> Result<String, io::Error> {
    let repo = repo::import()?;
    let temp_dir = repo.snap_dir.join("commit-temp");

    fs::create_dir_all(&temp_dir)?;

    let contents = fs::read_to_string(&repo.index_file)?;
    let files = contents.lines();

    for entry in files {
        let file_info = TreeEntry::from_string(entry);
        let path = Path::new(&file_info.name);
        if let Some(parent) = path.parent() {
            track_object(parent)?
        }
        track_object(path)?;
        let file_path = temp_dir.join(path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?
        }
        fs::write(file_path, file_info.hash.as_bytes())?
    }

    let tree = recurse_dir_commit(&temp_dir)?;
    let commit = Commit::new(
        branch::get_latest_commit()?,
        message.to_string(),
        author.to_string(),
        tree.hash,
    );
    hash::create_hash_dir(&commit.hash, &repo.snaps_dir)?;
    commit.write_to_file(&repo.snaps_dir.join(commit.get_hash_path()))?;

    fs::remove_dir_all(temp_dir)?;
    branch::update_branch(&commit.hash)?;

    Ok(commit.hash)
}
