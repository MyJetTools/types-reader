use std::collections::VecDeque;

use proc_macro2::{TokenStream, TokenTree};
use rust_extensions::StrOrString;

use crate::{NextToken, PeekedToken, ReferenceToken, TokenValue, TokensTreeExt};

pub struct TokensReader {
    token_stream: TokenStream,
    tokens: VecDeque<TokenTree>,
}

impl TokensReader {
    pub fn new(tokens: proc_macro2::TokenStream) -> Self {
        Self {
            token_stream: tokens.clone(),
            tokens: tokens.into_iter().collect(),
        }
    }

    pub fn try_peek_next_token(&mut self) -> Option<PeekedToken> {
        let result = self.tokens.front()?;

        Some(PeekedToken::from(result))
    }

    pub fn peek_next_token(&mut self, message: &str) -> Result<PeekedToken, syn::Error> {
        match self.tokens.front() {
            Some(token) => Ok(PeekedToken::from(token)),
            None => Err(syn::Error::new_spanned(&self.token_stream, message)),
        }
    }

    pub fn try_get_next_token(&mut self) -> Option<TokenTree> {
        self.tokens.pop_front()
    }

    pub fn get_next_token(
        &mut self,
        error_token: Option<&TokenTree>,
        msg: &str,
    ) -> Result<TokenTree, syn::Error> {
        match self.tokens.pop_front() {
            Some(token) => Ok(token),
            None => match error_token {
                Some(error_token) => Err(syn::Error::new_spanned(error_token, msg)),
                None => Err(syn::Error::new_spanned(&self.token_stream, msg)),
            },
        }
    }

    pub fn try_read_next_token(&mut self) -> Result<Option<NextToken>, syn::Error> {
        let next_token = self.try_peek_next_token();
        if next_token.is_none() {
            return Ok(None);
        }

        match next_token.unwrap() {
            PeekedToken::Ident => {
                let next_token = self.try_get_next_token().unwrap();
                Ok(Some(NextToken::Ident(next_token.unwrap_as_ident()?)))
            }
            PeekedToken::Group(_) => {
                let next_token = self.try_get_next_token().unwrap();
                return Ok(Some(NextToken::Group(next_token.unwrap_as_group()?)));
            }
            PeekedToken::Punct(char) => match char {
                '-' => Ok(Some(NextToken::Literal(TokenValue::new(self)?))),
                '&' => Ok(Some(NextToken::Reference(ReferenceToken::new(self)?))),
                _ => {
                    let next_token = self.try_get_next_token().unwrap();
                    Ok(Some(NextToken::Punct(next_token.unwrap_as_punct()?)))
                }
            },
            PeekedToken::Literal => Ok(Some(NextToken::Literal(TokenValue::new(self)?))),
        }
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

    pub fn throw_error(&self, message: impl Into<StrOrString<'static>>) -> syn::Error {
        let message: StrOrString<'_> = message.into();
        syn::Error::new_spanned(&self.token_stream, message.as_str())
    }
}

impl Into<TokensReader> for TokenStream {
    fn into(self) -> TokensReader {
        TokensReader::new(self)
    }
}
