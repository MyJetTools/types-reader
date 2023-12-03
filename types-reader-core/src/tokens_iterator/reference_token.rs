use proc_macro2::{Punct, TokenStream};
use rust_extensions::StrOrString;

use crate::{LifeTimeToken, TokensReader, TokensTreeExt};

#[derive(Debug)]
pub struct ReferenceToken {
    start_token: Punct,
    life_time: Option<LifeTimeToken>,
}

impl ReferenceToken {
    pub fn new(token_reader: &mut TokensReader) -> Result<Self, syn::Error> {
        let start_token = token_reader.get_next_token(None, "Expected start of reference token")?;

        let start_token = start_token.unwrap_as_punct()?;

        if start_token.as_char() != '&' {
            return Err(syn::Error::new_spanned(start_token, "Expected '&'"));
        }

        let next_token = token_reader.peek_next_token("Expecting continute of the code")?;

        let next_punct = next_token.unwrap_as_punct_char();

        if next_punct.is_none() {
            return Ok(Self {
                start_token,
                life_time: None,
            });
        }

        let punct = next_punct.unwrap();

        if punct != '\'' {
            return Err(syn::Error::new_spanned(
                start_token,
                "Expected lifetime ['] token",
            ));
        }

        let life_time = LifeTimeToken::new(token_reader)?;

        Ok(Self {
            start_token,
            life_time: Some(life_time),
        })
    }

    pub fn throw_error(&self, msg: StrOrString<'static>) -> syn::Error {
        syn::Error::new_spanned(&self.start_token, msg.as_str())
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let start_token = &self.start_token;
        match &self.life_time {
            Some(life_time) => {
                let life_time_name = life_time.to_token_stream();
                quote::quote!(&#life_time_name)
            }
            None => quote::quote!(#start_token),
        }
    }
}

impl TryFrom<TokenStream> for ReferenceToken {
    type Error = syn::Error;

    fn try_from(value: TokenStream) -> Result<Self, Self::Error> {
        let mut tokens_reader = TokensReader::new(value);
        Self::new(&mut tokens_reader)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ReferenceToken;

    #[test]
    fn test_based_reference() {
        let src = proc_macro2::TokenStream::from_str("&Name").unwrap();

        let reference_token: ReferenceToken = src.try_into().unwrap();

        assert_eq!("&", reference_token.to_token_stream().to_string());
    }

    #[test]
    fn test_with_reference_and_lifetime() {
        let src = proc_macro2::TokenStream::from_str("&'s Name").unwrap();

        let reference_token: ReferenceToken = src.try_into().unwrap();

        assert_eq!("& 's", reference_token.to_token_stream().to_string());
    }
}
