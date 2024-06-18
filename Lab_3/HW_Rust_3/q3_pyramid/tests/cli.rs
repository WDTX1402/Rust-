use assert_cmd::Command;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn amogus() -> TestResult {
    let expected = fs::read_to_string("tests/output.txt")?;
    let output = Command::cargo_bin("q3_pyramid")?.arg("4").output()?;
    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, expected, "Test failed");


   Ok(())
}