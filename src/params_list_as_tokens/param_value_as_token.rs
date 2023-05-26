use std::str::FromStr;

use proc_macro2::{Literal, TokenStream};

use syn::Ident;

use crate::{ObjectsList, ParamsListAsTokens};

pub enum ParamValueAsToken {
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
        value: Box<ParamsListAsTokens>,
    },
    ObjectList {
        token_stream: TokenStream,
        value: ObjectsList,
    },
}

impl ParamValueAsToken {
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
            ParamValueAsToken::None(ident) => syn::Error::new_spanned(ident.clone(), message),
            ParamValueAsToken::SingleValueAsIdent { ident, .. } => {
                syn::Error::new_spanned(ident.clone(), message)
            }
            ParamValueAsToken::String { literal, .. } => {
                syn::Error::new_spanned(literal.clone(), message)
            }
            ParamValueAsToken::Number { literal, .. } => {
                syn::Error::new_spanned(literal.clone(), message)
            }
            ParamValueAsToken::Double { literal, .. } => {
                syn::Error::new_spanned(literal.clone(), message)
            }
            ParamValueAsToken::Bool { literal, .. } => {
                syn::Error::new_spanned(literal.clone(), message)
            }
            ParamValueAsToken::Object { token_stream, .. } => {
                syn::Error::new_spanned(token_stream.clone(), message)
            }
            ParamValueAsToken::ObjectList { token_stream, .. } => {
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
}
