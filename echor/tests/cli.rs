use std::fs;
use assert_cmd::Command;
use predicates::prelude::predicate;


type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(vec!["Hallo", "Echo"])
        .assert()
        .success();

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    test_echo_result(vec!["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    test_echo_result(vec!["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_n() -> TestResult {
    test_echo_result(vec!["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_n() -> TestResult {
    test_echo_result(vec!["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn test_echo_result(args: Vec<&str>, outfile: &str) -> TestResult {

    let expected = fs::read_to_string(outfile)?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
