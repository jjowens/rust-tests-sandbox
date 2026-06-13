#[cfg(test)]
mod simple_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rust_tests_sandbox";

    #[test]
    fn should_return_hello_world() -> std::io::Result<()> {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let out = cmd.arg("hello-world").output().unwrap();

        Ok(())
    }

}