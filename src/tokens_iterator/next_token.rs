use proc_macro2::{Delimiter, TokenTree};

use crate::TokensReader;

pub struct NextToken {
    token_tree: TokenTree,
}

impl NextToken {
    pub fn new(token_tree: TokenTree) -> Self {
        Self { token_tree }
    }

    pub fn unwrap_into_ident(self, expected_sym: Option<&str>) -> Result<syn::Ident, syn::Error> {
        match self.token_tree {
            TokenTree::Ident(ident) => match expected_sym {
                Some(sym) => {
                    if ident.to_string() == sym {
                        return Ok(ident);
                    }

                    return Err(syn::Error::new_spanned(
                        ident,
                        format!("Expected {sym} Ident"),
                    ));
                }
                None => Ok(ident),
            },
            _ => Err(syn::Error::new_spanned(self.token_tree, "Expected Ident")),
        }
    }

    pub fn unwrap_into_group(
        self,
        expected_delimiter: Option<Delimiter>,
    ) -> Result<TokensReader, syn::Error> {
        match self.token_tree {
            TokenTree::Group(group) => match expected_delimiter {
                Some(delimiter) => {
                    if group.delimiter() == delimiter {
                        Ok(TokensReader::new(group.stream()))
                    } else {
                        return Err(syn::Error::new_spanned(
                            group,
                            format!("Expected Group with delimiter: {delimiter:?}"),
                        ));
                    }
                }
                None => Ok(TokensReader::new(group.stream())),
            },
            _ => Err(syn::Error::new_spanned(
                self.token_tree,
                "Expected Group of tokens",
            )),
        }
    }
}
