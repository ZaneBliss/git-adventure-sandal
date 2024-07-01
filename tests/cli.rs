use std::{
    env::set_current_dir,
    path::Path,
};

use common::restore_root_dir;

mod common;

#[test]
fn sanity_check_with_rspec() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("tests/fixtures/sanity_check_with_rspec");
    set_current_dir(path)?;

    let mut cmd = assert_cmd::Command::cargo_bin("teva")?;

    insta::with_settings!({ filters => vec![
        (r"\b\d*\.\d+\b", "[TIME]"),
        (r"33m[a-z0-9]{7}", "[SHA]")
    ]}, {
        insta::assert_snapshot!(String::from_utf8(cmd.output().unwrap().stdout).unwrap());
    });

    restore_root_dir()?;

    Ok(())
}

#[test]
fn not_enough_commits() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("tests/fixtures/not_enough_commits");
    set_current_dir(path)?;

    let mut cmd = assert_cmd::Command::cargo_bin("teva")?;

    insta::with_settings!({ filters => vec![
        (r"\b\d*\.\d+\b", "[TIME]"),
        (r"33m[a-z0-9]{7}", "[SHA]")
    ]}, {
        insta::assert_snapshot!(String::from_utf8(cmd.output().unwrap().stdout).unwrap());
    });

    restore_root_dir()?;

    Ok(())
}
