use grrs::line_contains;

#[test]
fn line_contains_returns_true_if_pattern_exists_in_line() {
    assert_eq!(line_contains("hello world", "world", false), true);
    assert_eq!(line_contains("hello world", "hello", false), true);
}

#[test]
fn line_contains_returns_true_if_pattern_exists_in_line_with_case_insensitive_matching() {
    assert_eq!(line_contains("Hello wOrld", "World", false), true);
}

#[test]
fn line_contains_returns_false_if_pattern_exists_in_line_with_case_sensitive_matching() {
    assert_eq!(line_contains("Hello wOrld", "World", true), false);
}

#[test]
fn test_line_contains_returns_false_if_pattern_is_not_found_in_line() {
    assert_eq!(line_contains("Hello", "test", false), false);
}
