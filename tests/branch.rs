use snappy::branch::{branch, get_latest_commit};
use snappy::checkout::checkout;
use snappy::commit::commit;
use snappy::stage::stage;
use snappy::repo::init;
use std::fs::{create_dir_all, read_to_string, remove_dir_all, write};
use std::path::Path;

#[test]
fn test_branch() {
    let snap_dir = Path::new(".snappy");
    let head_file = snap_dir.join("HEAD");
    init(true);

    commit("Basic");

    branch("test");
    assert_eq!(read_to_string(head_file).unwrap(), "test");

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    create_dir_all(&new_dir).unwrap();
    write(&new_file, &new_data.as_bytes()).unwrap();

    stage(&new_file);
    let hash = commit("Add test-checkout-file");

    checkout("master");
    assert_eq!(new_dir.exists(), false);
    assert_eq!(new_file.exists(), false);

    checkout("test");
    assert_eq!(new_dir.exists(), true);
    assert_eq!(new_file.exists(), true);

    let contents = read_to_string(new_file).unwrap();
    remove_dir_all(new_dir).unwrap();
    assert_eq!(contents, new_data);
    assert_eq!(hash, get_latest_commit());
}
