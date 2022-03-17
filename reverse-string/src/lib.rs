pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String:from();
    }
    let mut chars = input.chars();
    return reverse(chars.as_str()).append(chars.next())
}
