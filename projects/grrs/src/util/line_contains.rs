pub fn line_contains(line: &str, pattern: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        return line.contains(&pattern);
    }
    return line.to_lowercase().contains(&pattern.to_lowercase());
}
