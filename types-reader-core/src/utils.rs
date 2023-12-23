pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();

    for c in name.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}
