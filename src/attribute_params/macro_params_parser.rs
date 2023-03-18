use super::Position;

pub struct AttrParamsParser<'s> {
    src: &'s [u8],
    pos: usize,
}

impl<'s> AttrParamsParser<'s> {
    pub fn new(src: &'s [u8]) -> Self {
        Self { src, pos: 0 }
    }

    fn find_beginning_of_key(&mut self) -> Option<usize> {
        for i in self.pos..self.src.len() {
            if self.src[i] <= 32 {
                continue;
            }

            if self.src[i] <= b',' {
                continue;
            }

            if self.src[i] <= b';' {
                continue;
            }

            return Some(i);
        }
        None
    }

    fn find_the_end_of_param_name(&mut self) -> usize {
        for i in self.pos..self.src.len() {
            if self.src[i] <= 32 {
                return i;
            }

            if self.src[i] == b':' {
                return i;
            }

            if self.src[i] == b'=' {
                return i;
            }

            if self.src[i] == b',' {
                return i;
            }

            if self.src[i] == b';' {
                return i;
            }

            if self.src[i] == b')' {
                return i;
            }
        }

        self.src.len()
    }

    fn find_param_name_separator(&mut self) -> usize {
        for i in self.pos..self.src.len() {
            if self.src[i] == b':' {
                return i;
            }
            if self.src[i] == b'=' {
                return i;
            }
        }
        panic!(
            "Can not find param separator. Pos: {} Line: {}",
            self.pos,
            std::str::from_utf8(&self.src[self.pos..]).unwrap()
        );
    }

    fn find_start_of_value(&mut self) -> usize {
        for i in self.pos..self.src.len() {
            if self.src[i] > 32 {
                return i;
            }
        }
        panic!(
            "Can not find start of value. Pos: {} Line: {}",
            self.pos,
            std::str::from_utf8(&self.src[self.pos..]).unwrap()
        );
    }

    fn find_end_of_value(&mut self) -> usize {
        let b = self.src[self.pos];
        self.pos += 1;

        if b == b'"' || b == b'\'' {
            return self.find_the_end_of_string_value(b);
        }

        if b == b'{' || b == b'[' {
            return self.find_the_end_of_struct(b);
        }

        return self.find_end_of_non_string();
    }

    fn find_the_end_of_string_value(&self, wrapper: u8) -> usize {
        let mut i = self.pos;
        while i < self.src.len() {
            if self.src[i] == wrapper {
                return i;
            }

            if self.src[i] == b'\\' {
                i += 1;
            }

            i += 1;
        }

        panic!(
            "Can not find the end of string value. String value separator wrapper is: {}",
            wrapper as u8
        );
    }

    fn find_the_end_of_struct(&self, wrapper: u8) -> usize {
        let mut i = self.pos;
        while i < self.src.len() {
            if self.src[i] == wrapper {
                return i;
            }

            i += 1;
        }

        panic!(
            "Can not find the end of string value. String value separator wrapper is: {}",
            wrapper as u8
        );
    }

    fn find_end_of_non_string(&self) -> usize {
        let mut i = self.pos;
        while i < self.src.len() {
            let b = self.src[i];
            if b <= 32 {
                return i - 1;
            }

            if b == b',' {
                return i - 1;
            }

            if b == b';' {
                return i - 1;
            }

            if b == b')' {
                return i - 1;
            }

            i += 1;
        }

        return self.src.len() - 1;
    }
}

impl<'s> Iterator for AttrParamsParser<'s> {
    type Item = (Position, Position);

    fn next(&mut self) -> Option<Self::Item> {
        let param_name_start_pos = self.find_beginning_of_key()?;

        check_if_value_start_compliant_symbol(self.src[param_name_start_pos]);

        self.pos = param_name_start_pos;

        let param_name_end_pos = self.find_the_end_of_param_name();

        self.pos = param_name_end_pos + 1;

        if self.pos >= self.src.len() {
            return Some((
                Position {
                    from: param_name_start_pos,
                    to: param_name_end_pos,
                },
                Position { from: 0, to: 0 },
            ));
        }

        if self.src[param_name_end_pos] == b';' || self.src[param_name_end_pos] == b',' {
            return Some((
                Position {
                    from: param_name_start_pos,
                    to: param_name_end_pos,
                },
                Position { from: 0, to: 0 },
            ));
        }

        if self.src[param_name_end_pos] <= 32 {
            let param_name_separator = self.find_param_name_separator();
            self.pos = param_name_separator + 1;
        }

        let start_of_value = self.find_start_of_value();
        self.pos = start_of_value;

        let end_of_value = self.find_end_of_value();

        self.pos = end_of_value + 1;

        let key = Position {
            from: param_name_start_pos,
            to: param_name_end_pos,
        };

        let value = Position {
            from: start_of_value,
            to: end_of_value + 1,
        };

        return (key, value).into();
    }
}

fn check_if_value_start_compliant_symbol(c: u8) {
    if c >= b'a' && c <= b'z' {
        return;
    }

    if c >= b'A' && c <= b'Z' {
        return;
    }

    panic!(
        "Value should start from a latin digit. But it starts from {}",
        c as char
    );
}

#[cfg(test)]
mod test {
    use super::{AttrParamsParser, Position};

    #[test]
    pub fn test_simple_structure() {
        let params = r#"(a: "1", b: "2")"#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("b", key.get_str(params));
        assert_eq!("\"2\"", value.get_str(params));
    }

    #[test]
    pub fn test_simple_structure_but_separator_is_semi() {
        let params = r#"a: "1"; b: "2""#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("b", key.get_str(params));
        assert_eq!("\"2\"", value.get_str(params));
    }

    #[test]
    pub fn test_simple_structure_with_eq_as_separator() {
        let params = r#"a = "1", b="2""#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("b", key.get_str(params));
        assert_eq!("\"2\"", value.get_str(params));
    }

    #[test]
    pub fn test_number_and_bool() {
        let params = r#"(a:1, b=true)"#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("1", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("b", key.get_str(params));
        assert_eq!("true", value.get_str(params));
    }

    #[test]
    pub fn test_simple_structure_with_bool() {
        let params = r#"(a: "1", b: true)"#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("b", key.get_str(params));
        assert_eq!("true", value.get_str(params));
    }

    #[test]
    pub fn test_simple_structure_with_default_at_the_end() {
        let params = r#"(a: "1", default)"#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("default", key.get_str(params));
        assert_eq!("", value.get_str(params));
    }

    #[test]
    pub fn test_simple_structure_with_default_at_the_beginning() {
        let params = r#"(default, a: "1")"#;

        let result =
            AttrParamsParser::new(params.as_bytes()).collect::<Vec<(Position, Position)>>();

        let (key, value) = result.get(0).unwrap();
        assert_eq!("default", key.get_str(params));
        assert_eq!("", value.get_str(params));

        let (key, value) = result.get(1).unwrap();
        assert_eq!("a", key.get_str(params));
        assert_eq!("\"1\"", value.get_str(params));
    }
}
