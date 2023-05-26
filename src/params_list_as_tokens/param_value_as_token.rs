use proc_macro2::Literal;

use syn::Ident;

pub enum ParamValueAsToken {
    None(Ident),
    SingleValueAsIdent { ident: Ident, value: String },
    String { literal: Literal, value: String },
    Number { literal: Literal, value: i64 },
    Double { literal: Literal, value: f64 },
    Bool { literal: Literal, value: bool },
}

impl ParamValueAsToken {
    pub fn from_literal(literal: Literal) -> Result<Self, syn::Error> {
        let value = literal.to_string();

        if value.starts_with('"') || value.starts_with("'") {
            let value = value[1..value.len() - 1].to_string();
            return Ok(Self::String { literal, value });
        }

        if value.contains('.') {
            let value = value.parse::<f64>();
            match value {
                Ok(value) => return Ok(Self::Double { literal, value }),
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

        let value = value.parse::<i64>();
        match value {
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

            _ => Err(self.throw_error("Type is not string")),
        }
    }

    pub fn get_bool_value(&self) -> Result<bool, syn::Error> {
        match self {
            Self::Bool { value, .. } => Ok(*value),

            _ => Err(self.throw_error("Type is not string")),
        }
    }

    pub fn get_number_value(&self) -> Result<i64, syn::Error> {
        match self {
            Self::Number { value, .. } => Ok(*value),

            _ => Err(self.throw_error("Type is not string")),
        }
    }

    pub fn get_double_value(&self) -> Result<f64, syn::Error> {
        match self {
            Self::Double { value, .. } => Ok(*value),
            _ => Err(self.throw_error("Type is not string")),
        }
    }
}
