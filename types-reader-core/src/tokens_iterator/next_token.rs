use proc_macro2::{Delimiter, Group, Ident, Punct};
use rust_extensions::StrOrString;

use crate::{ReferenceToken, TokenValue, TokensReader};

#[derive(Debug)]
pub enum NextToken {
    Ident(Ident),
    Literal(TokenValue),
    Group(Group),
    Punct(Punct),
    Reference(ReferenceToken),
}

impl NextToken {
    pub fn try_unwrap_as_value(self) -> Result<TokenValue, Self> {
        match self {
            Self::Ident(value) => Err(Self::Ident(value)),
            Self::Literal(value) => Ok(value),
            Self::Group(value) => Err(Self::Group(value)),
            Self::Punct(value) => Err(Self::Punct(value)),
            Self::Reference(value) => Err(Self::Reference(value)),
        }
    }

    pub fn try_unwrap_into_ident(self) -> Result<syn::Ident, Self> {
        match self {
            Self::Ident(value) => Ok(value),
            Self::Literal(value) => Err(Self::Literal(value)),
            Self::Group(value) => Err(Self::Group(value)),
            Self::Punct(value) => Err(Self::Punct(value)),
            Self::Reference(value) => Err(Self::Reference(value)),
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
            Self::Group(group) => match expected_delimiter {
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
            _ => {}
        }
        Err(self.throw_error("Group is expected"))
    }

    pub fn try_unwrap_into_group(
        self,
        expected_delimiter: Option<Delimiter>,
    ) -> Result<(TokensReader, Delimiter), Self> {
        match self {
            NextToken::Group(group) => match expected_delimiter {
                Some(delimiter) => {
                    if group.delimiter() == delimiter {
                        return Ok((TokensReader::new(group.stream()), delimiter));
                    } else {
                        return Err(Self::Group(group));
                    }
                }
                None => return Ok((TokensReader::new(group.stream()), group.delimiter())),
            },
            Self::Ident(value) => Err(Self::Ident(value)),
            Self::Literal(value) => Err(Self::Literal(value)),
            Self::Punct(value) => Err(Self::Punct(value)),
            Self::Reference(value) => Err(Self::Reference(value)),
        }
    }

    pub fn if_spacing(&self, symbols: Option<&[char]>) -> bool {
        match self {
            Self::Punct(punct) => match symbols {
                Some(symbols) => {
                    for c in symbols {
                        if punct.as_char() == *c {
                            return true;
                        }
                    }

                    return false;
                }
                None => return true,
            },
            _ => {}
        }

        false
    }

    pub fn unwrap_into_spacing(self) -> Result<Punct, syn::Error> {
        match self {
            Self::Punct(punct) => Ok(punct),
            _ => Err(self.throw_error("Expected spacing")),
        }
    }

    pub fn throw_error(&self, message: impl Into<StrOrString<'static>>) -> syn::Error {
        let message: StrOrString<'_> = message.into();
        match self {
            Self::Ident(value) => syn::Error::new_spanned(value, message.as_str()),
            Self::Literal(value) => syn::Error::new_spanned(value.as_literal(), message.as_str()),
            Self::Group(value) => syn::Error::new_spanned(value, message.as_str()),
            Self::Punct(value) => syn::Error::new_spanned(value, message.as_str()),
            Self::Reference(value) => value.throw_error(message),
        }
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Ident(value) => quote::quote!(#value),
            Self::Literal(value) => value.to_token_stream(),
            Self::Group(value) => quote::quote!(#value),
            Self::Punct(value) => quote::quote!(#value),
            Self::Reference(value) => value.to_token_stream(),
        }
    }
}
