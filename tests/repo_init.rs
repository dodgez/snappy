extern crate snappy;

use std::path::Path;

#[test]
fn test_repo_init() {
    let snap_dir = Path::new(".snappy");
    snappy::repo::init(true);

    if !snap_dir.exists() {
      panic!("fatal: could not create snappy repository");
    }
}
