use proc_macro2::token_stream::IntoIter;

use crate::NextToken;

pub struct TokensReader {
    tokens: IntoIter,
}

impl TokensReader {
    pub fn new(tokens: proc_macro2::TokenStream) -> Self {
        Self {
            tokens: tokens.into_iter(),
        }
    }

    pub fn read_next_token(&mut self) -> NextToken {
        let next_token = self.tokens.next();
        if next_token.is_none() {
            panic!("Trying to read next token - but not tokens left");
        }

        NextToken::new(next_token.unwrap())
    }
}
