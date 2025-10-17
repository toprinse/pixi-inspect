use assert_cmd::Command;

#[test]
fn test_missing_file() {
    let mut cmd = Command::cargo_bin("pixi-inspect").unwrap();
    cmd.arg("get-info").arg("notfound.conda");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("pixi-inspect").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Extract metadata"));
}