use snappy::branch::get_latest_commit;
use snappy::commit::commit;
use snappy::repo::init;
use snappy::stage::stage;
use std::io;
use std::path::Path;

#[test]
fn test_commit() -> Result<(), io::Error> {
    init(true)?;

    stage(Path::new("./tests/commit.rs"))?;
    let hash = commit("Test message", "Author")?;
    assert_eq!(hash, get_latest_commit()?);

    Ok(())
}
