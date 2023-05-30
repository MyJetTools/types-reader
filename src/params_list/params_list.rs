use crate::{get_list_of_elements, SingleValueAsIdent};

use super::ParamValue;
use proc_macro2::{Ident, TokenStream, TokenTree};
use std::collections::{HashMap, VecDeque};

pub enum ParamsList {
    None(TokenStream),
    Single {
        token_stream: TokenStream,
        value: ParamValue,
    },
    Multiple {
        token_stream: TokenStream,
        items: HashMap<String, ParamValue>,
    },
}

impl ParamsList {
    pub fn new(token_stream: TokenStream) -> Result<Self, syn::Error> {
        let mut tokens: VecDeque<TokenTree> = token_stream.clone().into_iter().collect();

        if tokens.len() == 0 {
            return Ok(Self::None(token_stream));
        }

        if tokens.len() == 1 {
            let token = tokens.pop_front().unwrap();
            match token {
                TokenTree::Group(el) => {
                    println!("Group: {}", el.to_string());
                    panic!("Single element can not be group")
                }
                TokenTree::Ident(ident) => {
                    let value = ident.to_string();
                    return Ok(Self::Single {
                        token_stream,
                        value: ParamValue::SingleValueAsIdent(SingleValueAsIdent::new(
                            ident, value,
                        )),
                    });
                }
                TokenTree::Punct(el) => {
                    println!("Punct: {:?}", el.to_string());
                    panic!("Single element can not be punct");
                }
                TokenTree::Literal(literal) => {
                    return Ok(Self::Single {
                        token_stream,
                        value: ParamValue::from_literal(literal)?,
                    });
                }
            }
        }

        let mut items = HashMap::new();

        while let Some(ident) = get_ident(&mut tokens)? {
            let key: String = ident.to_string();

            let value = tokens.pop_front();

            if value.is_none() {
                return Ok(Self::Single {
                    token_stream,
                    value: ParamValue::None(ident),
                });
            }

            let mut value = value.unwrap();

            if let TokenTree::Punct(_) = &value {
                value = tokens.pop_front().unwrap();
            }

            let value = into_value(ident, value)?;
            items.insert(key, value);
        }

        Ok(Self::Multiple {
            token_stream,
            items,
        })
    }

    pub fn create_empty(token_stream: TokenStream) -> Self {
        Self::None(token_stream)
    }
    pub fn get_single_param(&self) -> Result<&ParamValue, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),
            Self::Single { value, .. } => Ok(value),
            Self::Multiple { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Attribute has multiple params",
                ));
            }
        }
    }

    pub fn try_get_single_param(&self) -> Option<&ParamValue> {
        match self {
            Self::None(_) => None,
            Self::Single { value, .. } => Some(value),
            Self::Multiple { .. } => None,
        }
    }

    pub fn get_named_param(&self, param_name: &str) -> Result<&ParamValue, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),
            Self::Single { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Named fields are required",
                ));
            }
            Self::Multiple {
                token_stream,
                items,
            } => match items.get(param_name) {
                Some(value) => return Ok(value),
                None => {
                    return Err(syn::Error::new_spanned(
                        token_stream.clone(),
                        format!("Field '{}' is required", param_name),
                    ));
                }
            },
        }
    }

    pub fn try_get_named_param(&self, param_name: &str) -> Option<&ParamValue> {
        match self {
            Self::Multiple { items, .. } => items.get(param_name),
            _ => None,
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        match self {
            Self::Multiple { items, .. } => items.contains_key(param_name),
            _ => false,
        }
    }

    pub fn get_from_single_or_named(&self, param_name: &str) -> Result<&ParamValue, syn::Error> {
        if let Ok(result) = self.get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }

    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::None(token_stream) => token_stream.clone(),
            Self::Single { token_stream, .. } => token_stream.clone(),
            Self::Multiple { token_stream, .. } => token_stream.clone(),
        }
    }
}

fn get_ident(items: &mut VecDeque<TokenTree>) -> Result<Option<Ident>, syn::Error> {
    while let Some(token_tree) = items.pop_front() {
        match token_tree {
            TokenTree::Ident(value) => return Ok(Some(value)),
            TokenTree::Literal(value) => {
                let str = value.to_string();
                return Err(syn::Error::new_spanned(
                    value,
                    format!("Expected ident but got literal {} ", str),
                ));
            }
            TokenTree::Punct(_) => {}

            TokenTree::Group(value) => {
                let str = value.to_string();
                return Err(syn::Error::new_spanned(
                    value,
                    format!("Expected ident but got group {} ", str),
                ));
            }
        }
    }

    Ok(None)
}

fn into_value(ident: Ident, token_tree: TokenTree) -> Result<ParamValue, syn::Error> {
    match token_tree {
        TokenTree::Ident(value) => Err(syn::Error::new_spanned(value, "Value can not be ident")),
        TokenTree::Group(value) => match value.delimiter() {
            proc_macro2::Delimiter::Parenthesis => {
                panic!(
                    "Not implemented group with Delimiter = Parenthesis. Value {}",
                    value.to_string()
                )
            }
            proc_macro2::Delimiter::Brace => {
                let token_stream = value.stream();
                let value = ParamsList::new(token_stream.clone())?;
                let result = ParamValue::Object {
                    token_stream,
                    value: Box::new(value),
                };
                return Ok(result);
            }
            proc_macro2::Delimiter::Bracket => {
                let token_stream = value.stream();
                let value = get_list_of_elements(token_stream.clone())?;

                return Ok(value);
            }
            proc_macro2::Delimiter::None => {
                panic!(
                    "Not implemented group with Delimiter = None. Value {}",
                    value.to_string()
                )
            }
        },
        TokenTree::Punct(_) => {
            return Ok(ParamValue::None(ident));
        }
        TokenTree::Literal(value) => {
            return Ok(ParamValue::from_literal(value)?);
        }
    }
}
