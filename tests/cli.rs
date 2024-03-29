use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("no-time-to-read")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}
