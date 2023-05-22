use std::collections::HashMap;

use crate::{ObjectsIteratorShared, ParamContent};

pub struct ParamsIterator<'s> {
    payload: &'s [u8],
    pos: usize,
    opener: u8,
}

impl<'s> ParamsIterator<'s> {
    pub fn new(src: &'s str) -> Self {
        let payload = src.as_bytes();
        let opener = super::utils::get_opener(payload);

        Self {
            payload,
            pos: 1,
            opener,
        }
    }

    pub fn get_next(&mut self) -> Option<(&'s str, ParamContent<'s>)> {
        self.skip_separator()?;

        let param_name_start = self.find_param_name_start()?;
        let param_name_end = self.find_param_name_end()?;

        let name = &self.payload[param_name_start..param_name_end];

        let param_name = std::str::from_utf8(name).unwrap();

        let value_type = self.find_value_start()?;
        if value_type.is_empty() {
            return Some((param_name, ParamContent::Empty));
        }

        let value_start = self.pos;

        let value_end = self.find_value_end(&value_type)?;

        let value = std::str::from_utf8(&self.payload[value_start..value_end]).unwrap();

        self.pos += 1;

        match &value_type {
            ValueType::Number => Some((param_name, ParamContent::Value(value))),
            ValueType::String => Some((param_name, ParamContent::Value(value))),
            ValueType::Bool => Some((param_name, ParamContent::Value(value))),
            ValueType::Array => Some((param_name, ParamContent::Array(value))),
            ValueType::Object => Some((param_name, ParamContent::Object(value))),
            ValueType::Empty => Some((param_name, ParamContent::Empty)),
        }
    }

    pub fn into_has_map(&mut self) -> HashMap<&'s str, ParamContent<'s>> {
        let mut result = HashMap::new();

        while let Some((name, value)) = self.get_next() {
            result.insert(name, value);
        }

        result
    }
}

impl<'s> ObjectsIteratorShared for ParamsIterator<'s> {
    fn get_current_byte(&self) -> Option<u8> {
        self.payload.get(self.pos).copied()
    }
    fn get_payload_len(&self) -> usize {
        self.payload.len()
    }

    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_opener(&self) -> u8 {
        self.opener
    }

    fn inc_pos(&mut self) {
        self.pos += 1;
    }
}

pub enum ValueType {
    Number,
    String,
    Bool,
    Array,
    Object,
    Empty,
}

impl ValueType {
    pub fn is_empty(&self) -> bool {
        match self {
            ValueType::Empty => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParamsIterator;

    #[test]
    fn test_detecting_basic_types() {
        let test = r#"(param1="My Value", param2: 253.2; default; param3:true, param4: {a:1, b:'2'}, param5: [{a:1, b:'2'}, {a:1, b:'2'}])"#;

        let mut iterator = ParamsIterator::new(test);

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("param1", param.0);
        assert_eq!("\"My Value\"", param.1.as_raw_str());
        assert!(param.1.is_value());

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("param2", param.0);
        assert_eq!("253.2", param.1.as_raw_str());
        assert!(param.1.is_value());

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("default", param.0);
        assert!(param.1.is_empty());

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("param3", param.0);
        assert_eq!("true", param.1.as_raw_str());
        assert!(param.1.is_value());

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("param4", param.0);
        assert_eq!("{a:1, b:'2'}", param.1.as_raw_str());
        assert!(param.1.is_object());

        let param = iterator.get_next().unwrap();

        println!("{:?}", param);

        assert_eq!("param5", param.0);
        assert_eq!("[{a:1, b:'2'}, {a:1, b:'2'}]", param.1.as_raw_str());
        assert!(param.1.is_array());

        let param = iterator.get_next();
        println!("{:?}", param);

        assert!(param.is_none());
    }
}
