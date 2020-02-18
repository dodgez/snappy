use snappy::repo::init;
use std::path::Path;

#[test]
fn test_repo_init() {
    let snap_dir = Path::new(".snappy");
    match init(true) {
      Ok(_) => (),
      Err(e) => panic!(e),
    }

    if !snap_dir.exists() {
      panic!("fatal: could not create snappy repository");
    }
}
