use std::{
    env::{self, set_current_dir},
    path::Path,
};

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

    set_current_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()))?;

    Ok(())
}
