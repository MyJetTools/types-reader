use proc_macro2::{token_stream::IntoIter, TokenStream};

use crate::NextToken;

pub struct TokensReader {
    token_stream: TokenStream,
    tokens: IntoIter,
}

impl TokensReader {
    pub fn new(tokens: proc_macro2::TokenStream) -> Self {
        Self {
            token_stream: tokens.clone(),
            tokens: tokens.into_iter(),
        }
    }

    pub fn try_read_next_token(&mut self) -> Result<Option<NextToken>, syn::Error> {
        let next_token = self.tokens.next();
        if next_token.is_none() {
            return Ok(None);
        }

        let next_token = next_token.unwrap();

        if let proc_macro2::TokenTree::Punct(value) = &next_token {
            if value.as_char() == '-' {
                match self.tokens.next() {
                    Some(second_token) => {
                        if let proc_macro2::TokenTree::Literal(second_token) = second_token {
                            return Ok(Some(NextToken::AsLiteralWithNegativeValue(second_token)));
                        } else {
                            return Err(syn::Error::new_spanned(
                                value,
                                "Literal must be after '-' token",
                            ));
                        }
                    }
                    None => {
                        return Err(syn::Error::new_spanned(
                            value,
                            "Something must go After '-' token",
                        ));
                    }
                }
            }
        }

        Ok(Some(NextToken::Single(next_token)))
    }

    pub fn read_next_token(&mut self) -> Result<NextToken, syn::Error> {
        let next_token = self.try_read_next_token()?;
        if next_token.is_none() {
            panic!("Trying to read next token - but no tokens left");
        }

        Ok(next_token.unwrap())
    }

    pub fn into_token_stream(self) -> TokenStream {
        self.token_stream
    }
}

impl Into<TokensReader> for TokenStream {
    fn into(self) -> TokensReader {
        TokensReader::new(self)
    }
}
