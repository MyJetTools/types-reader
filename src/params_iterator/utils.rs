use crate::ValueType;

pub fn is_param_name_separator(b: u8) -> bool {
    b == b':' || b == b'='
}

pub fn is_value_separator(b: u8) -> bool {
    b == b';' || b == b','
}

pub fn is_beginning_of_value(b: u8) -> Option<ValueType> {
    if b <= 32 {
        return None;
    }
    if b == b':' {
        return None;
    }

    if b == b'=' {
        return None;
    }

    if is_value_separator(b) {
        return Some(ValueType::Empty);
    }

    if b >= b'0' && b <= b'9' {
        return Some(ValueType::Number);
    }

    if b == b'+' || b == b'-' {
        return Some(ValueType::Number);
    }

    if b == b't' || b == b'T' || b == b'f' || b == b'F' {
        return Some(ValueType::Bool);
    }

    if b == b'[' {
        return Some(ValueType::Array);
    }

    if b == b'{' {
        return Some(ValueType::Object);
    }

    if b == b'\'' || b == b'"' {
        return Some(ValueType::String);
    }

    panic!("Value can not be started form '{}'", b as char);
}

pub fn is_closer(opener: u8, b: u8) -> bool {
    if opener == b'(' {
        return b == b')';
    }

    if opener == b'{' {
        return b == b'}';
    }

    if opener == b'[' {
        return b == b']';
    }

    panic!("Invalid Object Opener: '{}'", opener as char);
}

pub fn get_opener(src: &[u8]) -> u8 {
    if src[0] == b'(' {
        return src[0];
    }

    if src[0] == b'[' {
        return src[0];
    }

    if src[0] == b'{' {
        return src[0];
    }
    panic!("Invalid parameters object opener: '{}'", src[0] as char);
}
