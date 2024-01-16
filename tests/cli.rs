use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnot_exist() -> Result<(), Box<dyn std::error::Error>>{

    let mut command = Command::cargo_bin("command-line-grep-tool")?;

    command.arg("check").arg("../text.txt");

    command.assert()
    .failure()
    .stderr(predicate::str::contains("Could not read file \'../text.txt\'"));
        
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("An ant\n come to its nest\n bringing more sugar")?;

    // Act
    let mut command = Command::cargo_bin("command-line-grep-tool")?;
    command.arg("nest").arg(file.path());

    // Assert
    command.assert()
    .success()
    .stdout(predicate::str::contains("come to its nest"));

    Ok(())
}
