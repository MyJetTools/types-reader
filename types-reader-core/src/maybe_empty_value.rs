use crate::{ObjectValue, OptionalObjectValue};

#[derive(Clone)]
pub enum MaybeEmptyValue<T: Clone> {
    Empty,
    WithValue(T),
}

impl<'s, T: Clone + TryFrom<&'s ObjectValue, Error = syn::Error>> MaybeEmptyValue<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::WithValue(_) => false,
        }
    }

    pub fn unwrap_ref_of_value(&self) -> &T {
        match self {
            Self::Empty => panic!("EmptyValue::Empty"),
            Self::WithValue(value) => value,
        }
    }

    pub fn from_optional_object_value(value: &'s OptionalObjectValue) -> Result<Self, syn::Error> {
        match value {
            OptionalObjectValue::Empty(_) => Ok(MaybeEmptyValue::Empty),
            OptionalObjectValue::None(_) => Ok(MaybeEmptyValue::Empty),
            OptionalObjectValue::SingleValue(value) => {
                Ok(MaybeEmptyValue::WithValue(value.try_into()?))
            }
            OptionalObjectValue::Value { value, .. } => {
                Ok(MaybeEmptyValue::WithValue(value.try_into()?))
            }
        }
    }
}
