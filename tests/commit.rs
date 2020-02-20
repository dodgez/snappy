use snappy::{branch, commit, repo, stage};
use std::io;
use std::path::Path;

#[test]
fn test_commit() -> Result<(), io::Error> {
    repo::init(true)?;

    stage::stage(Path::new("./tests/commit.rs"))?;
    let hash = commit::commit("Test message", "Author")?;
    assert_eq!(hash, branch::get_latest_commit()?);

    Ok(())
}
