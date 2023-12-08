use crate::{AnyValueAsStr, BoolValue, DoubleValue, NumberValue, ObjectValue, StringValue};

#[derive(Debug)]
pub enum OptionalObjectValue {
    Empty(proc_macro2::TokenStream),
    None(syn::Ident),
    SingleValue(ObjectValue),
    Value {
        name: syn::Ident,
        value: ObjectValue,
    },
}

impl OptionalObjectValue {
    pub fn get_len(&self) -> usize {
        match self {
            Self::Empty(_) => 0,
            Self::None(_) => 0,
            Self::SingleValue(_) => 1,
            Self::Value { .. } => 1,
        }
    }
    pub fn unwrap_value(&self) -> Result<&ObjectValue, syn::Error> {
        match self {
            Self::Empty(itm) => Err(syn::Error::new_spanned(
                itm,
                "Expecting value but found no value",
            )),
            Self::None(itm) => Err(syn::Error::new_spanned(
                itm,
                "Expecting value but found no value",
            )),
            Self::SingleValue(value) => Ok(value),
            Self::Value { value, .. } => Ok(value),
        }
    }

    pub fn has_no_value(&self) -> bool {
        match self {
            Self::Empty(_) => true,
            Self::None(_) => true,
            _ => false,
        }
    }

    pub fn throw_error_at_ident(&self, message: &str) -> syn::Error {
        match self {
            Self::Empty(id) => syn::Error::new_spanned(id, message),
            Self::None(ident) => syn::Error::new_spanned(ident, message),
            Self::SingleValue(value) => value.throw_error(message),
            Self::Value { name, .. } => syn::Error::new_spanned(name, message),
        }
    }

    pub fn try_get_single_param(&self) -> Option<&ObjectValue> {
        match self {
            Self::SingleValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        match self {
            Self::Empty(id) => syn::Error::new_spanned(id, message),
            Self::None(ident) => syn::Error::new_spanned(ident, message),
            Self::SingleValue(value) => value.throw_error(message),
            Self::Value { value, .. } => value.throw_error(message),
        }
    }

    pub fn as_string(&self) -> Result<&StringValue, syn::Error> {
        match self {
            Self::Empty(id) => Err(syn::Error::new_spanned(id, "Expecting String value")),
            Self::None(ident) => Err(syn::Error::new_spanned(
                ident,
                "Expecting String value but found no value",
            )),
            Self::SingleValue(value) => value.as_string(),
            Self::Value { value, .. } => value.as_string(),
        }
    }

    pub fn as_number(&self) -> Result<&NumberValue, syn::Error> {
        match self {
            Self::Empty(id) => Err(syn::Error::new_spanned(id, "Expecting Number value")),
            Self::None(ident) => Err(syn::Error::new_spanned(
                ident,
                "Expecting Number value but found no value",
            )),
            Self::SingleValue(value) => value.as_number(),
            Self::Value { value, .. } => value.as_number(),
        }
    }

    pub fn as_double(&self) -> Result<&DoubleValue, syn::Error> {
        match self {
            Self::Empty(id) => Err(syn::Error::new_spanned(id, "Expecting Float value")),
            Self::None(ident) => Err(syn::Error::new_spanned(
                ident,
                "Expecting Float value but found no value",
            )),
            Self::SingleValue(value) => value.as_double(),
            Self::Value { value, .. } => value.as_double(),
        }
    }

    pub fn unwrap_any_value_as_str(&self) -> Result<&dyn AnyValueAsStr, syn::Error> {
        match self {
            Self::Empty(id) => Err(syn::Error::new_spanned(id, "No value here")),
            Self::None(ident) => Err(syn::Error::new_spanned(ident, "No value here")),
            Self::SingleValue(value) => Ok(value),
            Self::Value { value, .. } => Ok(value),
        }
    }

    pub fn as_bool(&self) -> Result<&BoolValue, syn::Error> {
        match self {
            Self::Empty(id) => Err(syn::Error::new_spanned(id, "Expecting Bool value")),
            Self::None(ident) => Err(syn::Error::new_spanned(
                ident,
                "Expecting Bool value but found no value",
            )),
            Self::SingleValue(value) => value.as_bool(),
            Self::Value { value, .. } => value.as_bool(),
        }
    }
}

impl<'s> TryInto<&'s ObjectValue> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s ObjectValue, Self::Error> {
        self.unwrap_value()
    }
}

impl<'s> TryInto<&'s str> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        let value = self.as_string()?.as_str();
        Ok(value)
    }
}

impl<'s> TryInto<String> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let value = self.as_string()?.as_str();
        Ok(value.to_string())
    }
}

impl<'s> TryInto<i8> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i8, Self::Error> {
        let value = self.as_number()?.as_i8();
        Ok(value)
    }
}

impl<'s> TryInto<u8> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        let value = self.as_number()?.as_u8();
        Ok(value)
    }
}

impl<'s> TryInto<i16> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i16, Self::Error> {
        let value = self.as_number()?.as_i16();
        Ok(value)
    }
}

impl<'s> TryInto<u16> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u16, Self::Error> {
        let value = self.as_number()?.as_u16();
        Ok(value)
    }
}

impl<'s> TryInto<i32> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        let value = self.as_number()?.as_i32();
        Ok(value)
    }
}

impl<'s> TryInto<u32> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        let value = self.as_number()?.as_u32();
        Ok(value)
    }
}

impl<'s> TryInto<i64> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        let value = self.as_number()?.as_i64();
        Ok(value)
    }
}

impl<'s> TryInto<u64> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        let value = self.as_number()?.as_u64();
        Ok(value)
    }
}

impl<'s> TryInto<isize> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<isize, Self::Error> {
        let value = self.as_number()?.as_isize();
        Ok(value)
    }
}

impl<'s> TryInto<usize> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let value = self.as_number()?.as_usize();
        Ok(value)
    }
}

impl<'s> TryInto<f32> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<f32, Self::Error> {
        let value = self.as_double()?.as_f32();
        Ok(value)
    }
}

impl<'s> TryInto<f64> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        let value = self.as_double()?.as_f64();
        Ok(value)
    }
}
