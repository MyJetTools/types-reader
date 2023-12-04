use std::ops::Deref;

use crate::{ObjectValue, ValueAsIdent};

pub struct AnyValueAsStr<'s> {
    value: &'s ObjectValue,
}

impl<'s> AnyValueAsStr<'s> {
    pub fn new(value: &'s ObjectValue) -> Self {
        Self { value }
    }
    pub fn as_str(&self) -> &str {
        let result = match &self.value {
            ObjectValue::String(value) => value.as_str(),
            ObjectValue::Number(value) => value.as_str(),
            ObjectValue::Double(value) => value.as_str(),
            ObjectValue::Bool(value) => value.as_str(),
            ObjectValue::Ident(value) => value.as_str(),
        };

        result
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        self.value.throw_error(message)
    }
}

impl<'s> TryInto<&'s str> for &'s AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        Ok(self.as_str())
    }
}

impl<'s> TryInto<String> for &'s AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let value = self.as_str();
        Ok(value.to_string())
    }
}

impl<'s> Deref for AnyValueAsStr<'s> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
