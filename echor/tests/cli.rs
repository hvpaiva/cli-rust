use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

#[test]
fn dies_with_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs_with_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello, world!").assert().success();
    Ok(())
}

#[test]
fn runs_hello1() -> Result<()> {
    run(&["Hello there"], "tests/fixtures/hello1.txt")
}

#[test]
fn runs_hello1_no_new_line() -> Result<()> {
    run(&["Hello there", "-n"], "tests/fixtures/hello1.n.txt")
}

#[test]
fn runs_hello2() -> Result<()> {
    run(&["Hello", "there"], "tests/fixtures/hello2.txt")
}

#[test]
fn runs_hello2_no_new_line() -> Result<()> {
    run(&["Hello", "there", "-n"], "tests/fixtures/hello2.n.txt")
}

#[test]
fn runs_hello3() -> Result<()> {
    run(&["Hello  there"], "tests/fixtures/hello3.txt")
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let cmd = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8(cmd.stdout).expect("Invalid UTF-8");
    assert_eq!(expected, stdout);
    Ok(())
}
