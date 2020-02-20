use std::fs::{create_dir_all, remove_dir_all, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Repo {
    pub snap_dir: PathBuf,
    pub snaps_dir: PathBuf,
    pub branches_dir: PathBuf,
    pub head_file: PathBuf,
    pub index_file: PathBuf,
    pub tracked_file: PathBuf,
    pub master_branch_file: PathBuf,
}

impl Repo {
    fn new() -> Repo {
        let snap_dir = Path::new(".snappy");
        let branches_dir = &snap_dir.join("branches");

        Repo {
            snap_dir: snap_dir.to_path_buf(),
            snaps_dir: snap_dir.join("snaps"),
            branches_dir: branches_dir.to_path_buf(),
            head_file: snap_dir.join("HEAD"),
            index_file: snap_dir.join("index"),
            tracked_file: snap_dir.join("tracked"),
            master_branch_file: branches_dir.join("master"),
        }
    }
}

pub fn init(force: bool) -> Result<Repo, io::Error> {
    let repo = Repo::new();
    if repo.snap_dir.exists() {
        if force {
            remove_dir_all(&repo.snap_dir)?;
        } else {
            panic!("fatal: found an existing snappy repository");
        }
    }

    create_dir_all(&repo.snap_dir)?;
    create_dir_all(&repo.snaps_dir)?;
    create_dir_all(&repo.branches_dir)?;

    File::create(&repo.head_file)?.write("master".as_bytes())?;
    File::create(&repo.index_file)?;
    File::create(&repo.tracked_file)?;
    File::create(&repo.master_branch_file)?.write("0".as_bytes())?;

    Ok(repo)
}

pub fn import() -> Result<Repo, io::Error> {
    let repo = Repo::new();
    if !repo.snap_dir.exists() {
        panic!("fatal: not a snappy repository");
    }

    if !repo.snaps_dir.exists() {
        create_dir_all(&repo.snaps_dir)?;
    }
    if !repo.branches_dir.exists() {
        create_dir_all(&repo.branches_dir)?;
    }
    if !repo.head_file.exists() {
        File::create(&repo.head_file)?.write("master".as_bytes())?;
    }
    if !repo.index_file.exists() {
        File::create(&repo.index_file)?;
    }
    if !repo.tracked_file.exists() {
        File::create(&repo.tracked_file)?;
    }
    if !repo.master_branch_file.exists() {
        File::create(&repo.master_branch_file)?.write("0".as_bytes())?;
    }

    Ok(repo)
}
