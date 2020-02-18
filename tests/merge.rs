use snappy::branch::get_latest_commit;
use snappy::checkout::checkout;
use snappy::commit::commit;
use snappy::merge::merge;
use snappy::repo::init;
use snappy::stage::stage;
use std::fs::{create_dir_all, remove_dir_all, write};
use std::io;
use std::path::Path;

#[test]
fn test_merge() -> Result<(), io::Error> {
    init(true)?;

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    create_dir_all(&new_dir)?;
    write(&new_file, &new_data.as_bytes())?;

    stage(&new_file)?;
    let hash = commit("Add test-checkout-file", "Author")?;

    write(&new_file, "".as_bytes())?;
    stage(&new_file)?;
    let branch_hash = commit("Delete test-checkout-file data", "Author")?;

    checkout(&hash)?;
    merge(&branch_hash)?;

    assert_eq!(get_latest_commit()?, branch_hash);

    remove_dir_all(new_dir)?;

    Ok(())
}
