use snappy::branch::get_latest_commit;
use snappy::checkout::checkout;
use snappy::commit::commit;
use snappy::stage::stage;
use snappy::repo::init;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

#[test]
fn test_checkout() {
    init(true);

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    create_dir_all(&new_dir).unwrap();
    write(&new_file, &new_data.as_bytes()).unwrap();

    stage(&new_file);
    let hash = commit("Add test-checkout-file");

    write(&new_file, "".as_bytes()).unwrap();
    stage(&new_file);
    commit("Delete test-checkout-file data");

    checkout(&hash);
    assert_eq!(hash, get_latest_commit());
    assert_eq!(read_to_string(new_file).unwrap(), new_data);
}
