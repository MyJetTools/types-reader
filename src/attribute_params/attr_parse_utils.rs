pub fn parse_attribute_with_name_and_params(src: &str) -> Result<(String, Option<String>), String> {
    if !src.starts_with("#") {
        return Err("Attribute has to start with #".to_string());
    }

    let src = &src[1..];

    let from = src.find('(');
    if from.is_none() {
        let name = src[1..src.len() - 1].trim().to_string();

        return Ok((name, None));
    }

    let from = from.unwrap();

    let to = find_from_end(src);

    if to.is_none() {
        panic!("Attribute does not have a closing bracket");
    }

    let to = to.unwrap();

    if to - from == 1 {
        return Ok((src[1..from].to_string(), None));
    }

    Ok((
        src[1..from].trim().to_string(),
        Some(src[from + 1..to].to_string()),
    ))
}

fn find_from_end(src: &str) -> Option<usize> {
    let mut i = src.len() - 1;

    let as_bytes = src.as_bytes();

    while i > 0 {
        if as_bytes[i] == b')' {
            return Some(i);
        }
        i -= 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::parse_attribute_with_name_and_params;

    #[test]
    fn test_case_with_param_as_string() {
        let attr = r#"#[operator(">")]"#;

        let (name, params) = parse_attribute_with_name_and_params(attr).unwrap();

        assert_eq!("operator", name);
        assert_eq!("\">\"", params.unwrap());
    }

    #[test]
    fn test_case_with_no_params_and_no_brackets() {
        let attr = r#"#[operator]"#;

        let (name, params) = parse_attribute_with_name_and_params(attr).unwrap();

        assert_eq!("operator", name);
        assert_eq!(true, params.is_none());
    }

    #[test]
    fn test_case_with_no_params_and() {
        let attr = r#"#[operator()]"#;

        let (name, params) = parse_attribute_with_name_and_params(attr).unwrap();

        assert_eq!("operator", name);
        assert_eq!(true, params.is_none());
    }

    #[test]
    fn test_case_with_boolean_inside() {
        let attr = r#"#[operator(true)]"#;

        let (name, params) = parse_attribute_with_name_and_params(attr).unwrap();

        assert_eq!("operator", name);
        assert_eq!("true", params.unwrap());
    }
}
