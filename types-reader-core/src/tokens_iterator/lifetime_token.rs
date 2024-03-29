use std::str::FromStr;

use crate::{TokensReader, TokensTreeExt};

#[derive(Debug, Clone)]
pub struct LifeTimeToken {
    name: syn::Ident,
    as_string: String,
}

impl LifeTimeToken {
    pub fn new(token_reader: &mut TokensReader) -> Result<Self, syn::Error> {
        let next_token = token_reader.get_next_token(None, "Expected lifetime marker")?;

        let punct = next_token.unwrap_as_punct()?;

        if punct.as_char() != '\'' {
            return Err(syn::Error::new_spanned(punct, "Expected lifetime marker"));
        }

        let next_token = token_reader.get_next_token(None, "Expected lifetime name")?;

        let name = next_token.unwrap_as_ident()?;

        let as_string = name.to_string();

        Ok(Self { name, as_string })
    }

    pub fn as_str(&self) -> &str {
        &self.as_string
    }

    pub fn to_generic_before_ident_token_stream(&self) -> proc_macro2::TokenStream {
        proc_macro2::token_stream::TokenStream::from_str(format!("<'{}>", self.name).as_str())
            .unwrap()
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        proc_macro2::token_stream::TokenStream::from_str(format!("'{}", self.name).as_str())
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::TokensReader;

    #[test]
    fn test_parse_lifetime() {
        let src = proc_macro2::TokenStream::from_str("'a").unwrap();
        let mut tokens_reader = TokensReader::new(src);

        let token = LifeTimeToken::new(&mut tokens_reader).unwrap();

        assert_eq!("'a", token.to_token_stream().to_string());
    }
}
