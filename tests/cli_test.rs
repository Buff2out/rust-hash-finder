use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_invalid_n_zero() {
    let mut cmd = Command::cargo_bin("rust-hash-finder").unwrap();
    cmd.env("RUST_LOG", "off")
        .args(["-N", "0", "-F", "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_cli_invalid_f_zero() {
    let mut cmd = Command::cargo_bin("rust-hash-finder").unwrap();
    cmd.env("RUST_LOG", "off")
        .args(["-N", "3", "-F", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_cli_success() {
    let mut cmd = Command::cargo_bin("rust-hash-finder").unwrap();
    cmd.env("RUST_LOG", "off")
        .args(["-N", "3", "-F", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("000\""));
}
