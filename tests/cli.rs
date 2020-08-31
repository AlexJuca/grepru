use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn print_version() {
    let mut cmd = Command::cargo_bin("grepru").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("grepru 0.1.0"));
}