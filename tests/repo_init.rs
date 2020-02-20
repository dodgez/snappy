use snappy::repo;
use std::io;

#[test]
fn test_repo_init() -> Result<(), io::Error> {
    repo::init(true)?;

    Ok(())
}
