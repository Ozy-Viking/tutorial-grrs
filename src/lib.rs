pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let lines: Vec<&str> = Vec::from_iter(content.lines());
    for line in lines {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("Failed to write to writer.");
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("test\nfind", "test", &mut result);
    assert_eq!(result, b"test\n")
}
