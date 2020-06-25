use std::fs::{self, File};
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
    fn new(snap_dir: &Path) -> Repo {
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

pub fn init(bare: bool, force: bool) -> Result<Repo, io::Error> {
    let repo = if bare {
        Repo::new(&Path::new("."))
    } else {
        Repo::new(&Path::new(".snappy"))
    };
    if repo.snap_dir.exists() {
        if force && !bare {
            fs::remove_dir_all(&repo.snap_dir)?;
        } else {
            panic!("fatal: found an existing snappy repository");
        }
    }

    fs::create_dir_all(&repo.snap_dir)?;
    fs::create_dir_all(&repo.snaps_dir)?;
    fs::create_dir_all(&repo.branches_dir)?;

    File::create(&repo.head_file)?.write(b"master")?;
    File::create(&repo.index_file)?;
    File::create(&repo.tracked_file)?;
    File::create(&repo.master_branch_file)?.write(b"0")?;

    Ok(repo)
}

fn is_repo() -> bool {
    if Path::new(".snappy").exists() {
        return true;
    }

    let repo = Repo::new(&Path::new("."));
    if !repo.snaps_dir.exists() {
        return false;
    }

    true
}

pub fn is_bare() -> bool {
    !Path::new(".snappy").exists()
}

pub fn import() -> Result<Repo, io::Error> {
    if !is_repo() {
        panic!("fatal: not a snappy repository");
    }

    let mut snap_dir = Path::new(".snappy");
    if !snap_dir.exists() {
        snap_dir = Path::new(".");
    }

    let repo = Repo::new(&snap_dir);
    if !repo.snaps_dir.exists() {
        fs::create_dir_all(&repo.snaps_dir)?;
    }
    if !repo.branches_dir.exists() {
        fs::create_dir_all(&repo.branches_dir)?;
    }
    if !repo.head_file.exists() {
        File::create(&repo.head_file)?.write(b"master")?;
    }
    if !repo.index_file.exists() {
        File::create(&repo.index_file)?;
    }
    if !repo.tracked_file.exists() {
        File::create(&repo.tracked_file)?;
    }
    if !repo.master_branch_file.exists() {
        File::create(&repo.master_branch_file)?.write(b"0")?;
    }

    Ok(repo)
}
