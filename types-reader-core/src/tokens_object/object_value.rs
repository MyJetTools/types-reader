use std::str::FromStr;

use rust_extensions::StrOrString;

use crate::{
    AnyValueAsStr, BoolValue, DoubleValue, NumberValue, StringValue, TokenValue, ValueAsIdent,
};

#[derive(Debug)]
pub enum ObjectValue {
    Ident(ValueAsIdent),
    String(StringValue),
    Number(NumberValue),
    Double(DoubleValue),
    Bool(BoolValue),
}

impl ObjectValue {
    pub fn throw_error(&self, message: &str) -> syn::Error {
        match self {
            Self::Ident(value) => value.throw_error(message),
            Self::String(value) => value.throw_error(message),
            Self::Number(value) => value.throw_error(message),
            Self::Double(value) => value.throw_error(message),
            Self::Bool(value) => value.throw_error(message),
        }
    }
    pub fn as_string<'s>(&'s self) -> Result<&'s StringValue, syn::Error> {
        match self.try_as_string() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be a string")),
        }
    }

    pub fn try_as_string<'s>(&'s self) -> Option<&'s StringValue> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Result<&BoolValue, syn::Error> {
        match self.try_as_bool() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be bool")),
        }
    }

    pub fn try_as_bool(&self) -> Option<&BoolValue> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn try_as_ident(&self) -> Option<&ValueAsIdent> {
        match self {
            Self::Ident(value) => Some(value),
            _ => None,
        }
    }
    pub fn as_ident(&self) -> Result<&ValueAsIdent, syn::Error> {
        match self.try_as_ident() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be ident")),
        }
    }

    pub fn as_number(&self) -> Result<&NumberValue, syn::Error> {
        match self.try_as_number() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Value should be a number")),
        }
    }

    pub fn try_as_number(&self) -> Option<&NumberValue> {
        match self {
            Self::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Result<&DoubleValue, syn::Error> {
        match self.try_as_double() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be a double value")),
        }
    }

    pub fn try_as_double(&self) -> Option<&DoubleValue> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn parse<TResult: FromStr>(
        &self,
        err_msg: Option<impl Into<StrOrString<'static>>>,
    ) -> Result<TResult, syn::Error> {
        let value = match self {
            Self::String(value) => StrOrString::create_as_str(value.as_str()),
            Self::Number(value) => StrOrString::create_as_str(value.as_str()),
            Self::Double(value) => StrOrString::create_as_str(value.as_str()),
            Self::Bool(value) => StrOrString::create_as_str(value.as_str()),
            Self::Ident(value) => StrOrString::create_as_str(value.as_str()),
        };

        let value = value.as_str();

        match value.parse::<TResult>() {
            Ok(value) => Ok(value),
            Err(_) => match err_msg {
                Some(err_msg) => {
                    let err_msg = err_msg.into();
                    return Err(self.throw_error(err_msg.as_str()));
                }
                None => {
                    return Err(self.throw_error(format!("Can not parse value: {}", value).as_str()))
                }
            },
        }
    }

    pub fn any_value_as_str<'s>(&'s self) -> &'s dyn AnyValueAsStr<'s> {
        self
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<ObjectValue> for ObjectValue {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl TryInto<ObjectValue> for syn::Ident {
    type Error = syn::Error;

    fn try_into(self) -> Result<ObjectValue, Self::Error> {
        let value = self.to_string();

        if value == "true" {
            return Ok(ObjectValue::Bool(BoolValue::new(self, true, value)));
        }

        if value == "false" {
            return Ok(ObjectValue::Bool(BoolValue::new(self, false, value)));
        }

        Ok(ObjectValue::Ident(ValueAsIdent::new(self, value)))
    }
}

impl TryInto<ObjectValue> for TokenValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<ObjectValue, Self::Error> {
        let is_negative = self.is_negative();
        let mut value = self.to_string();

        if value.starts_with('"') || value.starts_with("'") {
            let value = value[1..value.len() - 1].to_string();
            return Ok(ObjectValue::String(StringValue::new(self, value)));
        }

        if value.contains('.') {
            let result = value.parse::<f64>();
            match result {
                Ok(mut double_value) => {
                    if is_negative {
                        double_value = -double_value;
                        value.insert(0, '-');
                    }
                    return Ok(ObjectValue::Double(DoubleValue::new(
                        self,
                        double_value,
                        value,
                    )));
                }
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        self.as_literal(),
                        "Value can not be parsed as double",
                    ));
                }
            }
        }

        match value.parse::<i64>() {
            Ok(mut i64_value) => {
                if is_negative {
                    i64_value = -i64_value;
                    value.insert(0, '-');
                }
                return Ok(ObjectValue::Number(NumberValue::new(
                    self, i64_value, value,
                )));
            }
            Err(_) => {
                return Err(syn::Error::new_spanned(self.as_literal(), "Unknown type"));
            }
        }
    }
}

impl<'s> AnyValueAsStr<'s> for ObjectValue {
    fn as_str(&'s self) -> &str {
        let result = match self {
            ObjectValue::String(value) => value.as_str(),
            ObjectValue::Number(value) => value.as_str(),
            ObjectValue::Double(value) => value.as_str(),
            ObjectValue::Bool(value) => value.as_str(),
            ObjectValue::Ident(value) => value.as_str(),
        };

        result
    }

    fn throw_error(&self, message: &str) -> syn::Error {
        self.throw_error(message)
    }
}

impl<'s> TryInto<&'s str> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        let value = self.as_string()?.as_str();
        Ok(value)
    }
}

impl<'s> TryInto<String> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let value = self.as_string()?.as_str();
        Ok(value.to_string())
    }
}

impl<'s> TryInto<i8> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i8, Self::Error> {
        let value = self.as_number()?.as_i8();
        Ok(value)
    }
}

impl<'s> TryInto<u8> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        let value = self.as_number()?.as_u8();
        Ok(value)
    }
}

impl<'s> TryInto<i16> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i16, Self::Error> {
        let value = self.as_number()?.as_i16();
        Ok(value)
    }
}

impl<'s> TryInto<u16> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u16, Self::Error> {
        let value = self.as_number()?.as_u16();
        Ok(value)
    }
}

impl<'s> TryInto<i32> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        let value = self.as_number()?.as_i32();
        Ok(value)
    }
}

impl<'s> TryInto<u32> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        let value = self.as_number()?.as_u32();
        Ok(value)
    }
}

impl<'s> TryInto<i64> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        let value = self.as_number()?.as_i64();
        Ok(value)
    }
}

impl<'s> TryInto<u64> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        let value = self.as_number()?.as_u64();
        Ok(value)
    }
}

impl<'s> TryInto<isize> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<isize, Self::Error> {
        let value = self.as_number()?.as_isize();
        Ok(value)
    }
}

impl<'s> TryInto<usize> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let value = self.as_number()?.as_usize();
        Ok(value)
    }
}

impl<'s> TryInto<f32> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<f32, Self::Error> {
        let value = self.as_double()?.as_f32();
        Ok(value)
    }
}

impl<'s> TryInto<f64> for &'s ObjectValue {
    type Error = syn::Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        let value = self.as_double()?.as_f64();
        Ok(value)
    }
}
