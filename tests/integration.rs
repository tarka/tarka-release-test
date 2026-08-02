use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_outputs_expected_sum() {
    // Run the built binary and assert expected stdout
    let mut cmd = Command::cargo_bin("tarka-release-test").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("2 + 2 = 4"));
}
