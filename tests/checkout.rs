use snappy::{branch, checkout, commit, repo, stage};
use std::path::Path;
use std::{fs, io};

#[test]
fn test_checkout() -> Result<(), io::Error> {
    repo::init(false, true)?;

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    fs::create_dir_all(&new_dir)?;
    fs::write(&new_file, &new_data.as_bytes())?;

    stage::stage(&new_file)?;
    let hash = commit::commit("Add test-checkout-file", "Author")?;

    fs::write(&new_file, "".as_bytes())?;
    stage::stage(&new_file)?;
    commit::commit("Delete test-checkout-file data", "Author")?;

    checkout::checkout(&hash)?;
    let contents = fs::read_to_string(new_file)?;
    fs::remove_dir_all(new_dir)?;
    assert_eq!(contents, new_data);
    assert_eq!(hash, branch::get_latest_commit()?);

    Ok(())
}
