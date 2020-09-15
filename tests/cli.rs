use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::{self, Write};
use tempfile::NamedTempFile;

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

#[test]
fn print_line_count() {
    let mut cmd = Command::cargo_bin("grepru").unwrap();
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "hi\nhi\nHello\nHello").unwrap();
    cmd.arg("hi").arg(file.path()).arg("-c");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn should_print_line_count_for_single_char_search() {
    let mut cmd = Command::cargo_bin("grepru").unwrap();
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "hi\nhi\nHello\nHello").unwrap();
    cmd.arg("h").arg(file.path()).arg("-c");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn should_print_line_number() {
    let mut cmd = Command::cargo_bin("grepru").unwrap();
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "hi\n hey\n").unwrap();

    cmd.arg("hey").arg(file.path()).arg("-n");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2:hey"));
}