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

    #[test]
    fn for_fibonacci_test() {
        let lines = test_file("./src/tests/cases/for.lox");

        assert_eq!(lines.len(), 21);

        assert_eq!(lines[0], "0");
        assert_eq!(lines[1], "1");
        assert_eq!(lines[2], "1");
        assert_eq!(lines[3], "2");
        assert_eq!(lines[4], "3");
        assert_eq!(lines[5], "5");
        assert_eq!(lines[6], "8");
        assert_eq!(lines[7], "13");
        assert_eq!(lines[8], "21");
        assert_eq!(lines[9], "34");
        assert_eq!(lines[10], "55");
        assert_eq!(lines[11], "89");
        assert_eq!(lines[12], "144");
        assert_eq!(lines[13], "233");
        assert_eq!(lines[14], "377");
        assert_eq!(lines[15], "610");
        assert_eq!(lines[16], "987");
        assert_eq!(lines[17], "1597");
        assert_eq!(lines[18], "2584");
        assert_eq!(lines[19], "4181");
        assert_eq!(lines[20], "6765");
    }

    #[test]
    fn recursive_function_test() {
        let lines = test_file("./src/tests/cases/fundef.lox");

        assert_eq!(lines.len(), 3);

        assert_eq!(lines[0], "1");
        assert_eq!(lines[1], "2");
        assert_eq!(lines[2], "3");
    }

    #[test]
    fn function_modifies_env_val_test() {
        let lines = test_file("./src/tests/cases/fun_mods_local_env.lox");

        assert_eq!(lines.len(), 1);

        assert_eq!(lines[0], "3");
    }

    #[test]
    fn function_with_return() {
        let lines = test_file("./src/tests/cases/funreturn.lox");

        assert_eq!(lines.len(), 1);

        assert_eq!(lines[0], "5");
    }

    #[test]
    fn function_with_empty_return() {
        let lines = test_file("./src/tests/cases/fun_empty_return.lox");

        assert_eq!(lines.len(), 2);

        assert_eq!(lines[0], "0");
        assert_eq!(lines[1], "nil");
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
