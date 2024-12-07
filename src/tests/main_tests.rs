#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn blocks_test() {
        let lines = test_file("./src/tests/cases/block.lox");

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "3");
        assert_eq!(lines[0], "3");
    }

    #[test]
    fn while_test() {
        let lines = test_file("./src/tests/cases/while.lox");

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "0");
    }

    #[test]
    fn while_math_test() {
        let lines = test_file("./src/tests/cases/while_math.lox");

        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], "10");
        assert_eq!(lines[1], "90");
        assert_eq!(lines[2], "720");
        assert_eq!(lines[3], "5040");
        assert_eq!(lines[4], "30240");
        assert_eq!(lines[5], "151200");
        assert_eq!(lines[6], "604800");
        assert_eq!(lines[7], "1814400");
        assert_eq!(lines[8], "3628800");
        assert_eq!(lines[9], "3628800");
    }

    fn test_file(file_path: &str) -> Vec<String> {
        let output = Command::new("cargo")
            .args(["run", file_path])
            .output()
            .unwrap()
            .stdout;

        std::str::from_utf8(&output)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }
}
