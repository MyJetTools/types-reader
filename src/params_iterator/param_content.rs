use crate::{ObjectsIterator, ParamsIterator};

#[derive(Debug)]
pub enum ParamContent<'s> {
    Value(&'s str),
    Object(&'s str),
    Array(&'s str),
    Empty,
}

impl<'s> ParamContent<'s> {
    pub fn as_raw_str(&self) -> &'s str {
        match self {
            ParamContent::Value(value) => value,
            ParamContent::Array(value) => value,
            ParamContent::Object(value) => value,
            ParamContent::Empty => "",
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            ParamContent::Value(_) => true,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            ParamContent::Value(_) => false,
            ParamContent::Array(_) => true,
            ParamContent::Object(_) => false,
            ParamContent::Empty => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            ParamContent::Value(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => true,
            ParamContent::Empty => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ParamContent::Value(_) => false,
            ParamContent::Array(_) => false,
            ParamContent::Object(_) => false,
            ParamContent::Empty => true,
        }
    }

    pub fn iterate_array_objects(&self) -> ObjectsIterator {
        match self {
            ParamContent::Value(value) => {
                panic!("Can not iterate array objects from value '{}'", value)
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
            ParamContent::Value(value) => {
                panic!("Can not iterate param fields from value '{}'", value)
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
