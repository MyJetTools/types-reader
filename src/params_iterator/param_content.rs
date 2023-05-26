use std::str::FromStr;

use crate::{ObjectsIterator, ParamsIterator};

#[derive(Debug, Clone)]
pub enum ParamContent<'s> {
    String(&'s str),
    Bool(&'s str),
    Number(&'s str),
    Object(&'s str),
    Array(&'s str),
    Empty,
}

impl<'s> ParamContent<'s> {
    pub fn as_raw_str(&self) -> &'s str {
        match self {
            ParamContent::String(value) => value,
            ParamContent::Bool(value) => value,
            ParamContent::Number(value) => value,
            ParamContent::Array(value) => value,
            ParamContent::Object(value) => value,
            ParamContent::Empty => "",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ParamContent::String(value) => &value[1..value.len() - 1],
            ParamContent::Bool(value) => value,
            ParamContent::Number(value) => value,
            ParamContent::Array(value) => value,
            ParamContent::Object(value) => value,
            ParamContent::Empty => "",
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            ParamContent::String(value) => {
                panic!("Can not convert string value '{}' to bool", value)
            }
            ParamContent::Bool(value) => {
                if value == &"true" {
                    true
                } else if value == &"false" {
                    false
                } else {
                    panic!("Can not convert string value '{}' to bool", value)
                }
            }
            ParamContent::Number(value) => {
                panic!("Can not convert number value '{}' to bool", value)
            }
            ParamContent::Array(value) => {
                panic!("Can not convert array value '{}' to bool", value)
            }
            ParamContent::Object(value) => {
                panic!("Can not convert object value '{}' to bool", value)
            }
            ParamContent::Empty => {
                panic!("Can not convert empty value to bool")
            }
        }
    }

    pub fn get_value<TResult: FromStr>(
        &'s self,
        err_message: Option<&'static str>,
    ) -> Result<TResult, String> {
        let value = self.as_str();
        match TResult::from_str(value) {
            Ok(result) => Ok(result),
            Err(_) => {
                if let Some(err) = err_message {
                    return Err(format!("{}", err));
                } else {
                    return Err(format!("Can not parse from string value: {}", value));
                }
            }
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            ParamContent::String(_) => true,
            ParamContent::Number(_) => false,
            ParamContent::Bool(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            ParamContent::String(_) => false,
            ParamContent::Number(_) => false,
            ParamContent::Bool(_) => true,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            ParamContent::String(_) => false,
            ParamContent::Number(_) => true,
            ParamContent::Bool(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            ParamContent::String(_) => false,
            ParamContent::Number(_) => false,
            ParamContent::Bool(_) => false,
            ParamContent::Array(_) => true,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            ParamContent::String(_) => false,
            ParamContent::Number(_) => false,
            ParamContent::Bool(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => true,
            ParamContent::Empty => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ParamContent::String(_) => false,
            ParamContent::Number(_) => false,
            ParamContent::Bool(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => true,
        }
    }

    pub fn iterate_array_objects(&self) -> ObjectsIterator {
        match self {
            ParamContent::String(value) => {
                panic!(
                    "Can not iterate array objects from string value '{}'",
                    value
                )
            }
            ParamContent::Number(value) => {
                panic!(
                    "Can not iterate array objects from number value '{}'",
                    value
                )
            }
            ParamContent::Bool(value) => {
                panic!("Can not iterate array objects from bool value '{}'", value)
            }
            ParamContent::Object(value) => {
                panic!("Can not iterate array objects from object '{}'", value)
            }
            ParamContent::Array(value) => ObjectsIterator::new(value),
            ParamContent::Empty => {
                panic!("Can not iterate array objects from Empty value")
            }
        }
    }

    pub fn iterate_object_params(&self) -> ParamsIterator {
        match self {
            ParamContent::String(value) => {
                panic!("Can not iterate param fields from string value '{}'", value)
            }
            ParamContent::Number(value) => {
                panic!("Can not iterate param fields from number value '{}'", value)
            }
            ParamContent::Bool(value) => {
                panic!("Can not iterate param fields from bool value '{}'", value)
            }
            ParamContent::Object(value) => return ParamsIterator::new(value),
            ParamContent::Array(_) => {
                panic!("Can not iterate param fields from array of objects. Please use iterate_array_objects")
            }
            ParamContent::Empty => {
                panic!("Can not iterate param fields from Empty value")
            }
        }
    }
}