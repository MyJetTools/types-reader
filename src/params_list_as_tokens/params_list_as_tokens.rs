use super::param_value_as_token::ParamValueAsToken;
use proc_macro2::{Ident, TokenStream, TokenTree};
use std::collections::{HashMap, VecDeque};

pub enum ParamData {
    None(TokenStream),
    Single(ParamValueAsToken),
    Multiple(HashMap<String, ParamValueAsToken>),
}

pub enum ParamsListAsTokens {
    None(TokenStream),
    Single {
        token_stream: TokenStream,
        value: ParamValueAsToken,
    },
    Multiple {
        token_stream: TokenStream,
        items: HashMap<String, ParamValueAsToken>,
    },
}

impl ParamsListAsTokens {
    pub fn new(token_stream: TokenStream) -> Result<Self, syn::Error> {
        let mut tokens: VecDeque<TokenTree> = token_stream.clone().into_iter().collect();

        if tokens.len() == 0 {
            return Ok(Self::None(token_stream));
        }

        if tokens.len() == 1 {
            let token = tokens.pop_front().unwrap();
            match token {
                TokenTree::Group(_) => {
                    panic!("Single element can not be group")
                }
                TokenTree::Ident(ident) => {
                    let value = ident.to_string();
                    return Ok(Self::Single {
                        token_stream,
                        value: ParamValueAsToken::SingleValueAsIdent { ident, value },
                    });
                }
                TokenTree::Punct(_) => {
                    panic!("Single element can not be separator");
                }
                TokenTree::Literal(literal) => {
                    return Ok(Self::Single {
                        token_stream,
                        value: ParamValueAsToken::from_literal(literal)?,
                    });
                }
            }
        }

        let mut items = HashMap::new();

        while let Some(ident) = tokens.pop_front() {
            let ident = into_ident(ident)?;

            let key: String = ident.to_string();

            let value = into_value(ident, tokens.pop_front())?;
            items.insert(key, value);
        }

        Ok(Self::Multiple {
            token_stream,
            items,
        })
    }

    pub fn get_single_param(&self) -> Result<&ParamValueAsToken, syn::Error> {
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

    pub fn get_named_param(&self, param_name: &str) -> Result<&ParamValueAsToken, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),
            Self::Single { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Does not have named params",
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
                        "Attribute has multiple params",
                    ));
                }
            },
        }
    }

    pub fn try_get_named_param(&self, param_name: &str) -> Option<&ParamValueAsToken> {
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

    pub fn get_from_single_or_named(
        &self,
        param_name: &str,
    ) -> Result<&ParamValueAsToken, syn::Error> {
        if let Ok(result) = self.get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }
}

fn into_ident(token_tree: TokenTree) -> Result<Ident, syn::Error> {
    match token_tree {
        TokenTree::Ident(value) => Ok(value),
        _ => Err(syn::Error::new_spanned(token_tree, "Expected ident")),
    }
}

fn into_value(
    ident: Ident,
    token_tree: Option<TokenTree>,
) -> Result<ParamValueAsToken, syn::Error> {
    if token_tree.is_none() {
        return Ok(ParamValueAsToken::None(ident));
    }
    match token_tree.unwrap() {
        TokenTree::Ident(value) => Err(syn::Error::new_spanned(value, "Value can not be ident")),
        TokenTree::Group(value) => {
            panic!("Not implemented {}", value.to_string())
        }
        TokenTree::Punct(_) => {
            return Ok(ParamValueAsToken::None(ident));
        }
        TokenTree::Literal(value) => {
            return Ok(ParamValueAsToken::from_literal(value)?);
        }
    }
}
