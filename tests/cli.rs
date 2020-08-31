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

#[test]
fn file_not_found() {
    let mut cmd = Command::cargo_bin("grepru").unwrap();
    cmd.arg("foo").arg("/tmp/grepru/tmp.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory (os error 2)"));
}