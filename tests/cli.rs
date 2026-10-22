#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn mock_cmd() -> Command {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        cmd.env("RUMOS_TEST_MOCK", "1");
        cmd
    }

    #[test]
    fn help_with_no_args() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn set_brightness_from_args() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["set", "50"])
            .assert()
            .success()
            .stdout(predicate::str::contains("50"));
        Ok(())
    }

    #[test]
    fn set_max_brightness() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.arg("max")
            .assert()
            .success()
            .stdout(predicate::str::contains("100"));
        Ok(())
    }

    #[test]
    fn set_min_brightness() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.arg("min")
            .assert()
            .success()
            .stdout(predicate::str::contains("5"));
        Ok(())
    }

    #[test]
    fn decrease_brightness() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["dec", "10"])
            .assert()
            .success()
            .stdout(predicate::str::contains("10"));
        Ok(())
    }

    #[test]
    fn increase_brightness() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["inc", "10"])
            .assert()
            .success()
            .stdout(predicate::str::contains("10"));
        Ok(())
    }

    #[test]
    fn quiet_argument() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["-q", "inc", "10"])
            .assert()
            .success()
            .stdout(predicate::str::is_empty());
        Ok(())
    }

    #[test]
    fn percent_argument() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["-p", "dec", "10"])
            .assert()
            .success()
            .stdout(predicate::str::contains("10"));
        Ok(())
    }

    #[test]
    fn list_devices() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.arg("list")
            .assert()
            .success()
            .stdout(predicate::str::contains("Mock devices"));
        Ok(())
    }

    #[test]
    fn get_with_devices() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["-d", "intel_backlight", "get"])
            .assert()
            .success()
            .stdout(predicate::str::contains("50"));
        Ok(())
    }

    #[test]
    fn flags_before_command_work() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["-p", "get"])
            .assert()
            .success()
            .stdout(predicate::str::contains("%"));
        Ok(())
    }

    #[test]
    fn flags_after_command_work() -> TestResult {
        let mut cmd = mock_cmd();
        cmd.args(["get", "-p"])
            .assert()
            .success()
            .stdout(predicate::str::contains("%"));
        Ok(())
    }
}
