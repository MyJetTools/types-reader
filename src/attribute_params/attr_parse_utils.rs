pub fn find_params(src: &str) -> Option<&str> {
    let from = src.find('(')?;
    let to = find_from_end(src)?;

    if to - from == 1 {
        return None;
    }

    Some(&src[from..to])
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
