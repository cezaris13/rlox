pub fn error(line: &i32, message: &str) {
    report(line, &"", message);
}

pub fn report(line: &i32, where_in_code: &str, message: &str) {
    println!("[line {line}] Error {where_in_code}: {message}");
    //   hadError = true;
}
