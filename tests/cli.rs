use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], filename: &str) -> TestResult {
    let outfile = "tests/expected/".to_string() + filename;
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("trecho")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("trecho")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "hello2.n.txt")
}
