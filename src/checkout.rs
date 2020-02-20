use std::{fs, io};
use std::path::Path;

use crate::{branch, hash, index, repo};
use crate::objects::{Commit, File, Tree};

fn populate_working_directory(hash: &str, partial_path: &Path) -> Result<(), io::Error> {
    let repo = repo::import()?;

    let hash_file = repo.snaps_dir.join(hash::get_hash_path(&hash));
    if !hash_file.exists() {
        panic!("fatal: object {} does not exist", &hash);
    }

    let contents = fs::read_to_string(hash_file)?;
    if contents.starts_with("file") {
        let file = File::from_string(&contents);
        index::update_index(&partial_path, &hash)?;
        fs::write(partial_path, file.contents)?;
    } else if contents.starts_with("tree") {
        if partial_path != Path::new("") {
            fs::create_dir_all(partial_path)?;
        }
        let tree = Tree::from_string(&contents);
        let partial_path = partial_path.to_path_buf();
        for child in tree.children {
            let path = partial_path.join(child.name);

            populate_working_directory(&child.hash, &path)?;
        }
    }

    Ok(())
}

pub fn checkout(commit_hash: &str) -> Result<(), io::Error> {
    let repo = repo::import()?;

    if repo.index_file.exists() {
        fs::remove_file(repo.index_file)?;
    }

    let mut commit = commit_hash.to_string();
    let branch_file = repo.branches_dir.join(commit_hash);
    if branch_file.exists() {
        commit = fs::read_to_string(branch_file)?;
    }
    let commit_file = repo.snaps_dir.join(hash::get_hash_path(&commit));
    let commit = Commit::from_file(&commit_file)?;
    let tree_hash = commit.tree;

    let tracked_contents = fs::read_to_string(repo.tracked_file)?;
    let mut tracked_objects = tracked_contents.lines();
    while let Some(object) = tracked_objects.next() {
        let path = Path::new(object);
        if path.exists() {
            if path.is_dir() {
                match fs::remove_dir(path) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            } else if path.is_file() {
                fs::remove_file(path)?;

                if let Some(parent) = path.parent() {
                    match fs::remove_dir(parent) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }
        }
    }

    populate_working_directory(&tree_hash, Path::new(""))?;

    branch::update_head(commit_hash)?;

    Ok(())
}
