use snappy::{branch, checkout, commit, repo, stage};
use std::path::Path;
use std::{fs, io};

#[test]
fn test_branch() -> Result<(), io::Error> {
    let repo = repo::init(false, true)?;

    commit::commit("Basic", "Author")?;

    branch::branch("test")?;
    assert_eq!(fs::read_to_string(repo.head_file)?, "test");

    let new_dir = Path::new("./test-checkout-folder/");
    let new_file = new_dir.join("test-checkout-file");
    let new_data = "Test data";
    fs::create_dir_all(&new_dir)?;
    fs::write(&new_file, &new_data.as_bytes())?;

    stage::stage(&new_file)?;
    let hash = commit::commit("Add test-checkout-file", "Author")?;

    checkout::checkout("master")?;
    assert_eq!(new_dir.exists(), false);
    assert_eq!(new_file.exists(), false);

    checkout::checkout("test")?;
    assert_eq!(new_dir.exists(), true);
    assert_eq!(new_file.exists(), true);

    let contents = fs::read_to_string(new_file)?;
    fs::remove_dir_all(new_dir)?;
    assert_eq!(contents, new_data);
    assert_eq!(hash, branch::get_latest_commit()?);

    Ok(())
}
