use crate::{
    BoolValue, DoubleValue, NumberValue, ObjectValue, OptionalObjectValue, StringValue,
    ValueAsIdent,
};

pub enum AnyValue<'s> {
    String(&'s StringValue),
    Bool(&'s BoolValue),
    Ident(&'s ValueAsIdent),
    Number(&'s NumberValue),
    Double(&'s DoubleValue),
    NoValue(&'s syn::Ident),
}

impl<'s> AnyValue<'s> {
    pub fn has_no_value(&self) -> bool {
        match self {
            Self::NoValue(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_as_ident(&'s self) -> Result<&'s ValueAsIdent, syn::Error> {
        match self {
            Self::Ident(value) => Ok(value),
            _ => Err(self.throw_error("Expected Ident")),
        }
    }

    pub fn unwrap_as_number(&'s self) -> Result<&'s NumberValue, syn::Error> {
        match self {
            Self::Number(value) => Ok(value),
            _ => Err(self.throw_error("Expected number value")),
        }
    }

    pub fn unwrap_as_double(&'s self) -> Result<&'s DoubleValue, syn::Error> {
        match self {
            Self::Double(value) => Ok(value),
            _ => Err(self.throw_error("Expected double value")),
        }
    }

    pub fn unwrap_as_bool(&'s self) -> Result<&'s BoolValue, syn::Error> {
        match self {
            Self::Bool(value) => Ok(value),
            _ => Err(self.throw_error("Expected boolean value")),
        }
    }

    pub fn unwrap_as_string(&'s self) -> Result<&'s StringValue, syn::Error> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(self.throw_error("Expected String Value")),
        }
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        match self {
            Self::NoValue(ident) => syn::Error::new_spanned(ident, message),
            Self::String(value) => value.throw_error(message),
            Self::Bool(value) => value.throw_error(message),
            Self::Ident(value) => value.throw_error(message),
            Self::Number(value) => value.throw_error(message),
            Self::Double(value) => value.throw_error(message),
        }
    }
}

impl<'s> TryInto<AnyValue<'s>> for &'s OptionalObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<AnyValue<'s>, Self::Error> {
        match self {
            OptionalObjectValue::Empty(ident) => Err(syn::Error::new_spanned(
                ident,
                "Expected value, found empty token stream",
            )),
            OptionalObjectValue::None(ident) => Ok(AnyValue::NoValue(ident)),
            OptionalObjectValue::SingleValue(value) => value.try_into(),
            OptionalObjectValue::Value { name: _, value } => value.try_into(),
        }
    }
}

impl<'s> TryInto<AnyValue<'s>> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<AnyValue<'s>, Self::Error> {
        match self {
            ObjectValue::String(value) => Ok(AnyValue::String(value)),
            ObjectValue::Bool(value) => Ok(AnyValue::Bool(value)),
            ObjectValue::Ident(value) => Ok(AnyValue::Ident(value)),
            ObjectValue::Number(value) => Ok(AnyValue::Number(value)),
            ObjectValue::Double(value) => Ok(AnyValue::Double(value)),
        }
    }
}
