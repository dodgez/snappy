use snappy::repo::init;
use std::io;

#[test]
fn test_repo_init() -> Result<(), io::Error> {
    init(true)?;

    Ok(())
}
