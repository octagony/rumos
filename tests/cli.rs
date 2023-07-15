#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), brightness::Error>;
    #[test]
    fn help_with_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn set_max_brightness() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        let expected = "Maximum brightness level reached (100%)\n";
        cmd.arg("max").assert().success().stdout(expected);
        Ok(())
    }

    #[test]
    fn set_min_brightness() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        let expected = "Minimum brightness level reached (5%)\n";
        cmd.arg("min").assert().success().stdout(expected);
        Ok(())
    }
}
