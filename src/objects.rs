use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::hash;

pub struct File {
    pub contents: String,
    pub hash: String,
}

pub struct TreeEntry {
    pub hash: String,
    pub name: String,
}

pub struct Tree {
    pub children: Vec<TreeEntry>,
    pub hash: String,
}

pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub parent: String,
    pub tree: String,
}

impl File {
    pub fn new(contents: String) -> File {
        let hash = hash::hash(&format!("file\n{}", contents));

        File { contents, hash }
    }

    pub fn from_file(path: &Path) -> Result<File, io::Error> {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = fs::read_to_string(path)?;
        Ok(File::from_string(&data))
    }

    pub fn from_string(contents: &str) -> File {
        let hash = hash::hash(contents);
        let contents = contents[5..].to_string();
        File { contents, hash }
    }

    pub fn get_hash_path(&self) -> PathBuf {
        hash::get_hash_path(&self.hash)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(path, format!("file\n{}", self.contents).as_bytes())?;

        Ok(())
    }
}

impl TreeEntry {
    pub fn from_string(data: &str) -> TreeEntry {
        let mut parts = data.split(':');
        let name = parts.next().unwrap().to_string();
        let hash = parts.next().unwrap().to_string();

        TreeEntry { hash, name }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.name, self.hash)
    }
}

impl Tree {
    pub fn new(children: Vec<TreeEntry>) -> Tree {
        let mut raw_children = Vec::<String>::new();
        for child in &children {
            raw_children.push(child.to_string());
        }
        let hash = hash::hash(&format!("tree\n{}", raw_children.join("\n")));

        Tree { children, hash }
    }

    pub fn _from_file(path: &Path) -> Result<Tree, io::Error> {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = fs::read_to_string(path)?;
        Ok(Tree::from_string(&data))
    }

    pub fn from_string(contents: &str) -> Tree {
        let hash = hash::hash(&contents);
        let mut lines = contents.lines();
        // Pass over identifier
        lines.next();
        let mut children = Vec::<TreeEntry>::new();
        for line in lines {
            children.push(TreeEntry::from_string(line));
        }

        Tree { children, hash }
    }

    pub fn get_hash_path(&self) -> PathBuf {
        hash::get_hash_path(&self.hash)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), io::Error> {
        let mut raw_children = Vec::<String>::new();
        for child in &self.children {
            raw_children.push(child.to_string());
        }

        fs::write(
            path,
            format!("tree\n{}", raw_children.join("\n")).as_bytes(),
        )?;

        Ok(())
    }
}

impl Commit {
    pub fn new(parent: String, message: String, author: String, tree: String) -> Commit {
        let hash = hash::hash(&format!(
            "commit\n{}\n{}\n{}\n{}",
            parent, message, author, tree
        ));

        Commit {
            parent,
            message,
            author,
            tree,
            hash,
        }
    }

    pub fn from_file(path: &Path) -> Result<Commit, io::Error> {
        if !path.exists() {
            panic!("fatal: object does not exist {}", path.display());
        }

        let data = fs::read_to_string(path)?;
        Ok(Commit::from_string(&data))
    }

    pub fn from_string(contents: &str) -> Commit {
        let hash = hash::hash(&contents);
        let mut lines = contents.lines();
        let _identifier = lines.next().unwrap();
        let parent = lines.next().unwrap().to_string();
        let message = lines.next().unwrap().to_string();
        let author = lines.next().unwrap().to_string();
        let tree = lines.next().unwrap().to_string();

        Commit {
            hash,
            message,
            author,
            parent,
            tree,
        }
    }

    pub fn get_hash_path(&self) -> PathBuf {
        hash::get_hash_path(&self.hash)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), io::Error> {
        fs::write(
            path,
            format!(
                "commit\n{}\n{}\n{}\n{}",
                self.parent, self.message, self.author, self.tree
            )
            .as_bytes(),
        )?;

        Ok(())
    }
}
