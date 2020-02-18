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
    match init(true) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }

    match commit("Basic", "Author") {
        Ok(_) => (),
        Err(e) => panic!(e),
    }

    match branch("test") {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
    assert_eq!(read_to_string(head_file).unwrap(), "test");

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    create_dir_all(&new_dir).unwrap();
    write(&new_file, &new_data.as_bytes()).unwrap();

    match stage(&new_file) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
    let hash = match commit("Add test-checkout-file", "Author") {
        Ok(latest_hash) => latest_hash,
        Err(e) => panic!(e),
    };

    match checkout("master") {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
    assert_eq!(new_dir.exists(), false);
    assert_eq!(new_file.exists(), false);

    match checkout("test") {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
    assert_eq!(new_dir.exists(), true);
    assert_eq!(new_file.exists(), true);

    let contents = read_to_string(new_file).unwrap();
    remove_dir_all(new_dir).unwrap();
    assert_eq!(contents, new_data);
    match get_latest_commit() {
        Ok(latest_hash) => assert_eq!(hash, latest_hash),
        Err(e) => panic!(e),
    }
}
