use proc_macro2::{Literal, Punct};

use crate::{TokensReader, TokensTreeExt};

#[derive(Debug)]
pub struct TokenValue {
    negative_value_token: Option<Punct>,
    inner: Literal,
}

impl TokenValue {
    pub fn new(tokens_reader: &mut TokensReader) -> Result<Self, syn::Error> {
        let next_token =
            tokens_reader.get_next_token(None, "Reading First Literal token failed")?;

        let (negative_value_token, inner) = if let proc_macro2::TokenTree::Punct(punct) = next_token
        {
            let next_token = tokens_reader
                .get_next_token(None, "Fail read value after reading negative sign token")?;

            (Some(punct), next_token.unwrap_as_literal()?)
        } else {
            (None, next_token.unwrap_as_literal()?)
        };

        Ok(Self {
            inner,
            negative_value_token,
        })
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn as_literal(&self) -> &Literal {
        &self.inner
    }

    pub fn is_negative(&self) -> bool {
        self.negative_value_token.is_some()
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let inner = &self.inner;
        if self.negative_value_token.is_some() {
            quote::quote!(-#inner)
        } else {
            quote::quote!(#inner)
        }
    }
}

impl TryInto<TokenValue> for proc_macro2::TokenStream {
    type Error = syn::Error;

    fn try_into(self) -> Result<TokenValue, Self::Error> {
        let mut tokens_reader = TokensReader::new(self);
        TokenValue::new(&mut tokens_reader)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::TokenValue;

    #[test]
    fn test_number() {
        let number_token = proc_macro2::TokenStream::from_str("12").unwrap();
        let token_value: TokenValue = number_token.try_into().unwrap();
        assert_eq!("12", token_value.to_token_stream().to_string());
    }

    #[test]
    fn test_negative_number() {
        let number_token = proc_macro2::TokenStream::from_str("-12").unwrap();
        let token_value: TokenValue = number_token.try_into().unwrap();
        assert_eq!("- 12", token_value.to_token_stream().to_string());
    }
}
