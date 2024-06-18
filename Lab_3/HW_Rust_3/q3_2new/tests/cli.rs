use assert_cmd::Command;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn sussy() -> TestResult {
    let expected = fs::read_to_string("tests/output.txt")?;
    let output = Command::cargo_bin("q3_2new")?.arg("4").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    assert_eq!(stdout, expected, "Test failed");

    Ok(())
}