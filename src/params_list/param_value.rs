use std::str::FromStr;

use proc_macro2::{Literal, TokenStream};

use rust_extensions::StrOrString;
use syn::Ident;

use crate::{
    BoolValue, DoubleValue, NumberValue, ParamsList, SingleValueAsIdent, StringValue, VecOfValues,
};

#[derive(Debug)]
pub enum ParamValue {
    None(Ident),
    SingleValueAsIdent(SingleValueAsIdent),
    String(StringValue),
    Number(NumberValue),
    Double(DoubleValue),
    Bool(BoolValue),
    Object {
        token_stream: TokenStream,
        value: Box<ParamsList>,
    },
    ObjectList {
        token_stream: TokenStream,
        value: Vec<ParamsList>,
    },
    VecOfValues(VecOfValues),
}

impl ParamValue {
    pub fn from_ident(ident: Ident) -> Result<Self, syn::Error> {
        let value = ident.to_string();

        if value == "true" {
            return Ok(Self::Bool(BoolValue::new(ident, true, value)));
        }

        if value == "false" {
            return Ok(Self::Bool(BoolValue::new(ident, false, value)));
        }
        return Err(syn::Error::new_spanned(value, "Unknown value ident"));
    }

    pub fn from_literal(literal: Literal, is_negative: bool) -> Result<Self, syn::Error> {
        let mut value = literal.to_string();

        if value.starts_with('"') || value.starts_with("'") {
            let value = value[1..value.len() - 1].to_string();
            return Ok(Self::String(StringValue::new(literal, value)));
        }

        if value.contains('.') {
            let result = value.parse::<f64>();
            match result {
                Ok(mut double_value) => {
                    if is_negative {
                        double_value = -double_value;
                        value.insert(0, '-');
                    }
                    return Ok(Self::Double(DoubleValue::new(literal, double_value, value)));
                }
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        literal,
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
                return Ok(Self::Number(NumberValue::new(literal, i64_value, value)));
            }
            Err(_) => {
                return Err(syn::Error::new_spanned(literal, "Unknown type"));
            }
        }
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        match self {
            Self::None(ident) => syn::Error::new_spanned(ident.clone(), message),
            Self::SingleValueAsIdent(value) => value.throw_error(message),
            Self::String(value) => value.throw_error(message),
            Self::Number(value) => value.throw_error(message),
            Self::Double(value) => value.throw_error(message),
            Self::Bool(value) => value.throw_error(message),
            Self::Object { token_stream, .. } => syn::Error::new_spanned(token_stream, message),
            Self::ObjectList { token_stream, .. } => syn::Error::new_spanned(token_stream, message),
            Self::VecOfValues(value) => value.throw_error(message),
        }
    }

    pub fn is_none(&self) -> Option<&Ident> {
        match self {
            Self::None(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_string_value<'s>(&'s self) -> Result<&'s StringValue, syn::Error> {
        match self.try_unwrap_as_string_value() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be a string")),
        }
    }

    pub fn try_unwrap_as_string_value<'s>(&'s self) -> Option<&'s StringValue> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_bool_value(&self) -> Result<&BoolValue, syn::Error> {
        match self.try_unwrap_as_bool_value() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be bool")),
        }
    }

    pub fn try_unwrap_as_bool_value(&self) -> Option<&BoolValue> {
        match self {
            Self::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_number_value(&self) -> Result<&NumberValue, syn::Error> {
        match self.try_unwrap_as_number_value() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Value should be a number")),
        }
    }

    pub fn try_unwrap_as_number_value(&self) -> Option<&NumberValue> {
        match self {
            Self::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_double_value(&self) -> Result<&DoubleValue, syn::Error> {
        match self.try_unwrap_as_double_value() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Type should be a double value")),
        }
    }

    pub fn try_unwrap_as_double_value(&self) -> Option<&DoubleValue> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_object_list(&self) -> Result<&Vec<ParamsList>, syn::Error> {
        match self.try_unwrap_as_object_list() {
            Some(value) => Ok(value),
            None => Err(self.throw_error("Value should be an object list")),
        }
    }

    pub fn try_unwrap_as_object_list(&self) -> Option<&Vec<ParamsList>> {
        match self {
            Self::ObjectList { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_vec_of_values(&self) -> Result<&VecOfValues, syn::Error> {
        match self.try_unwrap_as_vec_of_values() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Value should be a vector of string")),
        }
    }

    pub fn try_unwrap_as_vec_of_values(&self) -> Option<&VecOfValues> {
        match self {
            Self::VecOfValues(value) => Some(value),
            _ => None,
        }
    }

    pub fn unwrap_as_single_value(&self) -> Result<&SingleValueAsIdent, syn::Error> {
        match self.try_unwrap_as_single_value() {
            Some(value) => Ok(value),
            _ => Err(self.throw_error("Value should be a single value")),
        }
    }

    pub fn try_unwrap_as_single_value(&self) -> Option<&SingleValueAsIdent> {
        match self {
            Self::SingleValueAsIdent(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_value<TResult: FromStr>(
        &self,
        err_msg: Option<impl Into<StrOrString<'static>>>,
    ) -> Result<TResult, syn::Error> {
        let value = match self {
            Self::String(value) => StrOrString::create_as_str(value.as_str()),
            Self::Number(value) => StrOrString::create_as_str(value.as_str()),
            Self::Double(value) => StrOrString::create_as_str(value.as_str()),
            Self::Bool(value) => StrOrString::create_as_str(value.as_str()),
            _ => return Err(self.throw_error("Type should be a string")),
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

    pub fn get_any_value_as_str(&self) -> Result<&str, syn::Error> {
        let result = match self {
            Self::String(value) => value.as_str(),
            Self::Number(value) => value.as_str(),
            Self::Double(value) => value.as_str(),
            Self::Bool(value) => value.as_str(),
            _ => return Err(self.throw_error("Type should be a string")),
        };

        Ok(result)
    }

    pub fn is_vec_of_values(&self) -> bool {
        match self {
            Self::VecOfValues(_) => true,
            _ => false,
        }
    }
}
