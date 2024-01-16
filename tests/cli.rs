use assert_cmd::prelude::*;
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