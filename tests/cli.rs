#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    // TestType
    type TestResult = Result<(), brightness::Error>;

    // Helpers
    fn set_brightness_for_test() {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        cmd.args(["set", "50"]).assert().success();
    }

    // Tests
    #[test]
    fn help_with_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn set_brightness_from_args() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        let expected = "50";
        cmd.args(["set", expected])
            .assert()
            .success()
            .stdout(predicate::str::contains(expected));
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

    #[test]
    fn decrease_brightness() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        set_brightness_for_test();
        cmd.args(["dec", "10"])
            .assert()
            .success()
            .stdout(predicate::str::contains("40"));
        Ok(())
    }

    #[test]
    fn increase_brightness() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        set_brightness_for_test();
        cmd.args(["inc", "10"])
            .assert()
            .success()
            .stdout(predicate::str::contains("60"));
        Ok(())
    }

    #[test]
    fn quiet_argument() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        set_brightness_for_test();
        cmd.args(["-q", "inc", "10"]).assert().success().stdout("");
        Ok(())
    }

    #[test]
    fn percent_argument() -> TestResult {
        let mut cmd = Command::cargo_bin("rumos").unwrap();
        let expected = "40%\n";
        set_brightness_for_test();
        cmd.args(["-p", "dec", "10"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}
