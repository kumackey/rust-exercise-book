use assert_cmd::Command;

#[test]
fn fails() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure()
        .stderr(predicates::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}
