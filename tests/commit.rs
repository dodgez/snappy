use snappy::branch::get_latest_commit;
use snappy::commit::commit;
use snappy::stage::stage;
use snappy::repo::init;
use std::path::Path;

#[test]
fn test_commit() {
    init(true);

    stage(Path::new("./tests/commit.rs"));
    let hash = commit("Test message", "Author");
    assert_eq!(hash, get_latest_commit());
}
