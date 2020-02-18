use snappy::repo::init;
use std::io;
use std::path::Path;

#[test]
fn test_repo_init() -> Result<(), io::Error> {
    let snap_dir = Path::new(".snappy");
    init(true)?;

    if !snap_dir.exists() {
        panic!("fatal: could not create snappy repository");
    }

    Ok(())
}
