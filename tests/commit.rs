use snappy::branch::get_latest_commit;
use snappy::commit::commit;
use snappy::repo::init;
use snappy::stage::stage;
use std::path::Path;

#[test]
fn test_commit() {
    match init(true) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }

    match stage(Path::new("./tests/commit.rs")) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
    let hash = match commit("Test message", "Author") {
        Ok(commit_hash) => commit_hash,
        Err(e) => panic!(e),
    };
    match get_latest_commit() {
        Ok(latest_hash) => assert_eq!(hash, latest_hash),
        Err(e) => panic!(e),
    }
}
