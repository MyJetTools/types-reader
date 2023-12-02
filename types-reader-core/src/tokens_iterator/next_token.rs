use proc_macro2::{Delimiter, Literal, TokenTree};
use rust_extensions::StrOrString;

use crate::{TokenValue, TokensReader};

pub enum NextToken {
    Single(TokenTree),
    AsLiteralWithNegativeValue(Literal),
}

impl NextToken {
    pub fn new(token_tree: TokenTree) -> Self {
        Self::Single(token_tree)
    }

    pub fn try_unwrap_as_value(self) -> Result<TokenValue, Self> {
        match self {
            Self::Single(value) => match value {
                TokenTree::Literal(value) => Ok(TokenValue::new(value, false)),
                _ => Err(Self::Single(value)),
            },
            Self::AsLiteralWithNegativeValue(value) => Ok(TokenValue::new(value, true)),
        }
    }

    pub fn try_unwrap_into_ident(self) -> Result<syn::Ident, Self> {
        if self.is_literal_with_negative_value() {
            return Err(self);
        }

        match self {
            Self::Single(value) => match value {
                TokenTree::Ident(ident) => Ok(ident),
                _ => Err(Self::Single(value)),
            },
            Self::AsLiteralWithNegativeValue(_) => {
                panic!("Somehow we are here");
            }
        }
    }

    pub fn unwrap_into_ident(self, expected_sym: Option<&str>) -> Result<syn::Ident, syn::Error> {
        match self.try_unwrap_into_ident() {
            Ok(ident) => match expected_sym {
                Some(sym) => {
                    if ident.to_string() == sym {
                        return Ok(ident);
                    }

                    return Err(syn::Error::new_spanned(
                        ident,
                        format!("Expected Ident with content '{sym}'"),
                    ));
                }
                None => Ok(ident),
            },
            Err(err) => Err(err.throw_error("Expected ident")),
        }
    }

    pub fn unwrap_into_group(
        self,
        expected_delimiter: Option<Delimiter>,
    ) -> Result<TokensReader, syn::Error> {
        match self {
            Self::Single(value) => match value {
                TokenTree::Group(group) => match expected_delimiter {
                    Some(delimiter) => {
                        if group.delimiter() == delimiter {
                            return Ok(TokensReader::new(group.stream()));
                        } else {
                            return Err(syn::Error::new_spanned(
                                group,
                                format!("Expected Group with delimiter: {delimiter:?}"),
                            ));
                        }
                    }
                    None => return Ok(TokensReader::new(group.stream())),
                },
                _ => {
                    return Err(syn::Error::new_spanned(value, format!("Expected Group")));
                }
            },
            Self::AsLiteralWithNegativeValue(value) => {
                return Err(syn::Error::new_spanned(value, format!("Expected Group")));
            }
        }
    }

    pub fn try_unwrap_into_group(
        self,
        expected_delimiter: Option<Delimiter>,
    ) -> Result<(TokensReader, Delimiter), NextToken> {
        if self.is_literal_with_negative_value() {
            return Err(self);
        }

        match self {
            Self::Single(value) => match &value {
                TokenTree::Group(group) => match expected_delimiter {
                    Some(delimiter) => {
                        if group.delimiter() == delimiter {
                            return Ok((TokensReader::new(group.stream()), delimiter));
                        } else {
                            return Err(NextToken::Single(value));
                        }
                    }
                    None => return Ok((TokensReader::new(group.stream()), group.delimiter())),
                },
                _ => {
                    return Err(NextToken::Single(value));
                }
            },
            Self::AsLiteralWithNegativeValue(_) => {
                panic!("Somehow we are here")
            }
        }
    }

    pub fn if_spacing(&self, symbols: Option<&[char]>) -> bool {
        match self {
            Self::Single(token_tree) => {
                if let TokenTree::Punct(punct) = &token_tree {
                    match symbols {
                        Some(symbols) => {
                            for c in symbols {
                                if punct.as_char() == *c {
                                    return true;
                                }
                            }

                            return false;
                        }
                        None => return true,
                    }
                }
            }
            Self::AsLiteralWithNegativeValue(_) => {}
        }

        false
    }

    pub fn throw_error(&self, message: impl Into<StrOrString<'static>>) -> syn::Error {
        let message: StrOrString<'_> = message.into();
        match self {
            Self::Single(value) => syn::Error::new_spanned(value, message.as_str()),
            Self::AsLiteralWithNegativeValue(value) => {
                syn::Error::new_spanned(value, message.as_str())
            }
        }
    }

    pub fn is_literal_with_negative_value(&self) -> bool {
        matches!(self, Self::AsLiteralWithNegativeValue(_))
    }
}
