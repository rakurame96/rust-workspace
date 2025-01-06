use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use anyhow::Result;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("ch02_echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    let expected = fs::read_to_string("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("ch02_echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> Result<()> {
    let expected = fs::read_to_string("tests/expected/hello2.txt")?;
    let mut cmd = Command::cargo_bin("ch02_echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("ch02_echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// #[test]
// fn hello1() -> Result<()> {
//     run(&["Hello there"], "tests/expected/hello1.txt")
// }

// #[test]
// fn hello2() -> Result<()> {
    // run(&["Hello", "there"], "tests/expected/hello2.txt")
// }

#[test]
fn hello1_no_newline() -> Result<()> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt") // Two spaces!
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}