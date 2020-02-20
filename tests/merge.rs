use snappy::{branch, checkout, commit, merge, repo, stage};
use std::{fs, io};
use std::path::Path;

#[test]
fn test_merge() -> Result<(), io::Error> {
    repo::init(true)?;

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    fs::create_dir_all(&new_dir)?;
    fs::write(&new_file, &new_data.as_bytes())?;

    stage::stage(&new_file)?;
    let hash = commit::commit("Add test-checkout-file", "Author")?;

    fs::write(&new_file, "".as_bytes())?;
    stage::stage(&new_file)?;
    let branch_hash = commit::commit("Delete test-checkout-file data", "Author")?;

    checkout::checkout(&hash)?;
    merge::merge(&branch_hash)?;

    assert_eq!(branch::get_latest_commit()?, branch_hash);

    fs::remove_dir_all(new_dir)?;

    Ok(())
}
