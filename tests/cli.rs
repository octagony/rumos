use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_with_no_args() {
    let mut cmd = Command::cargo_bin("rumos").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
