#[cfg(test)]
mod simple_test {
    use assert_cmd::Command;
    const APP_NAME: &str = "rust_tests_sandbox";

    #[test]
    fn should_return_hello_world() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let output = cmd.arg("hello-world")
                                    .arg("--name").arg("Gary")
                                    .output().unwrap();

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        assert_eq!(output.status.success(), true);
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello, Gary!"));
    }

    #[test]
    fn should_return_hello_world_friend() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let output = cmd.arg("hello-world")
            .output().unwrap();

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        assert_eq!(output.status.success(), true);
        assert!(String::from_utf8_lossy(&output.stdout).contains("friend"));
    }

    #[test]
    fn short_name_should_error() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let output = cmd.arg("hello-world")
            .arg("--name").arg("L")
            .output().unwrap();

        println!("stdout: {}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(!output.status.success(), true);
        assert!(String::from_utf8_lossy(&output.stderr).contains("Name cannot be less than 2 characters or more than 10 characters"));

    }

    #[test]
    fn too_long_name_should_error() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let output = cmd.arg("hello-world")
            .arg("--name").arg("David23423459345323")
            .output().unwrap();

        println!("stdout: {}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(!output.status.success(), true);
        assert!(String::from_utf8_lossy(&output.stderr).contains("Name cannot be less than 2 characters or more than 10 characters"));

    }

    #[test]
    fn fred_not_allowed_error() {
        let mut cmd = Command::cargo_bin(APP_NAME).unwrap();

        let output = cmd.arg("hello-world")
            .arg("--name").arg("Fred")
            .output().unwrap();

        println!("stdout: {}", String::from_utf8_lossy(&output.stderr));
        assert_eq!(!output.status.success(), true);
        assert!(String::from_utf8_lossy(&output.stderr).contains("Fred cannot be greeted"));

    }

}