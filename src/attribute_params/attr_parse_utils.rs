pub fn find_params(src: &str) -> (String, Option<String>) {
    let from = src.find('(');
    if from.is_none() {
        panic!("Attribute does not have a name");
    }

    let from = from.unwrap();

    let to = find_from_end(src);

    if to.is_none() {
        panic!("Attribute does not have a closing bracket");
    }

    let to = to.unwrap();

    if to - from == 1 {
        return (src[..from].to_string(), None);
    }

    (src[1..from].to_string(), Some(src[from..to].to_string()))
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
