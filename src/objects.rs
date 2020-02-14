use std::fs::read_to_string;
use std::path::Path;

use crate::hash;

pub struct File {
    contents: String,
    hash: String,
}

pub struct Tree {
    children: Vec<String>,
    hash: String,
}

pub struct Commit {
    hash: String,
    message: String,
    parent: String,
    tree: String,
}

impl File {
    fn new(contents: String) -> File {
        let hash = hash::hash(&format!("file\n{}", contents));

        File {
            contents,
            hash,
        }
    }

    fn from_file(path: &Path) -> File {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = read_to_string(path).unwrap();
        let hash = hash::hash(&data);
        let contents = data[5..].to_string();
        File {
            contents,
            hash,
        }
    }
}

impl Tree {
    fn new(children: Vec<String>) -> Tree {
        let hash = hash::hash(&format!("tree\n{}", children.join("\n")));

        Tree { children, hash }
    }

    fn from_file(path: &Path) -> Tree {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = read_to_string(path).unwrap();
        let hash = hash::hash(&data);
        let mut lines = data.lines();
        // Pass over identifier
        lines.next();
        let mut children = Vec::<String>::new();
        while let Some(line) = lines.next() {
            children.push(line.to_string());
        }

        Tree {
            children,
            hash,
        }
    }
}

impl Commit {
    fn new(parent: String, message: String, tree: String) -> Commit {
        let hash = hash::hash(&format!("commit\n{}\n{}\n{}", parent, message, tree));

        Commit {
            parent,
            message,
            tree,
            hash,
        }
    }

    fn from_file(path: &Path) -> Commit {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = read_to_string(path).unwrap();
        let hash = hash::hash(&data);
        let mut lines = data.lines();
        let _identifier = lines.next().unwrap();
        let parent = lines.next().unwrap().to_string();
        let message = lines.next().unwrap().to_string();
        let tree = lines.next().unwrap().to_string();

        Commit {
            hash,
            message,
            parent,
            tree,
        }
    }
}
