pub fn find_matches(content:&str, pattern: &str, mut writer: impl std::io::Write){
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn test_find_a_match() {
    let mut result = Vec::new();
    find_matches("little known place has a lot to give", "has", &mut result);
    assert_eq!(result, b"little known place has a lot to give\n");
}