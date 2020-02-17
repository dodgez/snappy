use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write, File};
use std::path::Path;

use crate::branch::{get_latest_commit, update_branch};
use crate::hash;
use crate::objects::{Commit, Tree, TreeEntry};

fn track_object(path: &Path) {
    let snap_dir = Path::new(".snappy");
    let tracked_file = snap_dir.join("tracked");
    if !tracked_file.exists() {
        File::create(&tracked_file).unwrap();
    }

    let contents = read_to_string(&tracked_file).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if path == Path::new(line) {
            return;
        }
    }

    write(tracked_file, contents + &format!("{}\n", path.display())).unwrap();
}

fn recurse_dir_commit(path: &Path) -> TreeEntry {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");

    let mut contents = read_dir(path).unwrap();
    let mut children = Vec::<TreeEntry>::new();
    while let Some(dir_entry) = contents.next() {
        let entry_path = dir_entry.unwrap().path();

        if entry_path.is_dir() {
            children.push(recurse_dir_commit(
                &path.join(entry_path.file_name().unwrap()),
            ));
        } else if entry_path.is_file() {
            children.push(TreeEntry {
                hash: read_to_string(&entry_path).unwrap(),
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
    hash::create_hash_dir(&tree.hash, &snaps_dir);
    tree.write_to_file(&snaps_dir.join(tree.get_hash_path()));

    TreeEntry {
        name: path.file_name().unwrap().to_str().unwrap().to_owned(),
        hash: tree.hash,
    }
}

pub fn commit(message: &str) -> String {
    let snap_dir = Path::new(".snappy");
    let snaps_dir = snap_dir.join("snaps");
    let temp_dir = snap_dir.join("commit-temp");
    let index_file = snap_dir.join("index");
    if !snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    create_dir_all(&temp_dir).unwrap();

    let contents = read_to_string(&index_file).unwrap();
    let mut files = contents.lines();

    while let Some(entry) = files.next() {
        let file_info = TreeEntry::from_string(entry);
        let path = Path::new(&file_info.name);
        match path.parent() {
            Some(parent) => track_object(parent),
            None => (),
        }
        track_object(path);
        let file_path = temp_dir.join(path);
        match file_path.parent() {
            Some(parent) => create_dir_all(parent).unwrap(),
            None => (),
        }
        write(file_path, file_info.hash.as_bytes()).unwrap()
    }

    let tree = recurse_dir_commit(&temp_dir);
    let commit = Commit::new(get_latest_commit(), message.to_string(), tree.hash);
    hash::create_hash_dir(&commit.hash, &snaps_dir);
    commit.write_to_file(&snaps_dir.join(commit.get_hash_path()));

    remove_dir_all(temp_dir).unwrap();
    update_branch(&commit.hash);

    return commit.hash;
}
