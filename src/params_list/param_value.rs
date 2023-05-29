use std::str::FromStr;

use proc_macro2::{Literal, TokenStream};

use rust_extensions::StrOrString;
use syn::Ident;

use crate::ParamsList;

pub enum ParamValue {
    None(Ident),
    SingleValueAsIdent {
        ident: Ident,
        value: String,
    },
    String {
        literal: Literal,
        value: String,
    },
    Number {
        literal: Literal,
        value: i64,
    },
    Double {
        literal: Literal,
        value: String,
    },
    Bool {
        literal: Literal,
        value: bool,
    },
    Object {
        token_stream: TokenStream,
        value: Box<ParamsList>,
    },
    ObjectList {
        token_stream: TokenStream,
        value: Vec<ParamsList>,
    },
    VecOfString {
        token_stream: TokenStream,
        value: Vec<String>,
    },
}

impl ParamValue {
    pub fn from_literal(literal: Literal) -> Result<Self, syn::Error> {
        let value = literal.to_string();

        if value.starts_with('"') || value.starts_with("'") {
            let value = value[1..value.len() - 1].to_string();
            return Ok(Self::String { literal, value });
        }

        if value.contains('.') {
            let result = value.parse::<f64>();
            match result {
                Ok(_) => return Ok(Self::Double { literal, value }),
                Err(_) => {
                    return Err(syn::Error::new_spanned(
                        literal,
                        "Value can not be parsed as double",
                    ));
                }
            }
        }

        if value == "true" {
            return Ok(Self::Bool {
                literal,
                value: true,
            });
        }

        if value == "false" {
            return Ok(Self::Bool {
                literal,
                value: false,
            });
        }

        match value.parse::<i64>() {
            Ok(value) => return Ok(Self::Number { literal, value }),
            Err(_) => {
                return Err(syn::Error::new_spanned(literal, "Unknown type"));
            }
        }
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        match self {
            Self::None(ident) => syn::Error::new_spanned(ident.clone(), message),
            Self::SingleValueAsIdent { ident, .. } => {
                syn::Error::new_spanned(ident.clone(), message)
            }
            Self::String { literal, .. } => syn::Error::new_spanned(literal.clone(), message),
            Self::Number { literal, .. } => syn::Error::new_spanned(literal.clone(), message),
            Self::Double { literal, .. } => syn::Error::new_spanned(literal.clone(), message),
            Self::Bool { literal, .. } => syn::Error::new_spanned(literal.clone(), message),
            Self::Object { token_stream, .. } => {
                syn::Error::new_spanned(token_stream.clone(), message)
            }
            Self::ObjectList { token_stream, .. } => {
                syn::Error::new_spanned(token_stream.clone(), message)
            }

            Self::VecOfString { token_stream, .. } => {
                syn::Error::new_spanned(token_stream.clone(), message)
            }
        }
    }

    pub fn is_none(&self) -> Option<&Ident> {
        match self {
            Self::None(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_str_value(&self) -> Result<&str, syn::Error> {
        match self {
            Self::String { value, .. } => Ok(value),

            _ => Err(self.throw_error("Type should be a string")),
        }
    }

    pub fn get_str_value_token(&self) -> Result<TokenStream, syn::Error> {
        match self {
            Self::String { value, .. } => Ok(quote::quote! { #value }),
            _ => Err(self.throw_error("Type should be a string")),
        }
    }

    pub fn get_bool_value(&self) -> Result<bool, syn::Error> {
        match self {
            Self::Bool { value, .. } => Ok(*value),

            _ => Err(self.throw_error("Type should be bool")),
        }
    }

    pub fn get_bool_value_token(&self) -> Result<TokenStream, syn::Error> {
        match self {
            Self::Bool { value, .. } => match value {
                true => Ok(TokenStream::from_str("true").unwrap()),
                false => Ok(TokenStream::from_str("false").unwrap()),
            },

            _ => Err(self.throw_error("Type should be bool")),
        }
    }

    pub fn get_number_value(&self) -> Result<i64, syn::Error> {
        match self {
            Self::Number { value, .. } => Ok(*value),

            _ => Err(self.throw_error("Type should be a number")),
        }
    }

    pub fn get_number_value_token(&self) -> Result<Literal, syn::Error> {
        match self {
            Self::Number { value, .. } => Ok(Literal::i64_unsuffixed(*value)),

            _ => Err(self.throw_error("Type should be a number")),
        }
    }

    pub fn get_double_value(&self) -> Result<f64, syn::Error> {
        match self {
            Self::Double { value, .. } => Ok(value.parse::<f64>().unwrap()),
            _ => Err(self.throw_error("Type should be a double value")),
        }
    }

    pub fn get_double_value_token(&self) -> Result<TokenStream, syn::Error> {
        match self {
            Self::Double { value, .. } => Ok(TokenStream::from_str(value).unwrap()),
            _ => Err(self.throw_error("Type should be a double value")),
        }
    }

    pub fn unwrap_as_object_list(&self) -> Result<&Vec<ParamsList>, syn::Error> {
        match self {
            Self::ObjectList { value, .. } => Ok(value),
            _ => Err(self.throw_error("Value should be an object list")),
        }
    }

    pub fn unwrap_as_vec_of_string(&self) -> Result<&Vec<String>, syn::Error> {
        match self {
            Self::VecOfString { value, .. } => Ok(value),
            _ => Err(self.throw_error("Value should be a vector of string")),
        }
    }

    pub fn get_value<TResult: FromStr>(
        &self,
        err_msg: Option<impl Into<StrOrString<'static>>>,
    ) -> Result<TResult, syn::Error> {
        let value = match self {
            Self::String { value, .. } => StrOrString::create_as_str(value.as_str()),
            Self::Number { value, .. } => StrOrString::create_as_string(value.to_string()),
            Self::Double { value, .. } => StrOrString::create_as_string(value.to_string()),
            Self::Bool { value, .. } => StrOrString::create_as_string(value.to_string()),
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

    pub fn get_any_value_as_string(&self) -> Result<StrOrString, syn::Error> {
        let result = match self {
            Self::String { value, .. } => StrOrString::create_as_str(value.as_str()),
            Self::Number { value, .. } => StrOrString::create_as_string(value.to_string()),
            Self::Double { value, .. } => StrOrString::create_as_string(value.to_string()),
            Self::Bool { value, .. } => StrOrString::create_as_string(value.to_string()),
            _ => return Err(self.throw_error("Type should be a string")),
        };

        Ok(result)
    }
}
