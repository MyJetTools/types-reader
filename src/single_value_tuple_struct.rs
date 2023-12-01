use proc_macro2::{Delimiter, Ident};

use crate::TokensReader;

pub struct SingleValueTupleStruct {
    pub is_public: bool,
    pub name_ident: Ident,
    pub type_ident: Ident,
}

impl SingleValueTupleStruct {
    pub fn new(input: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
        let mut tokens_iterator = TokensReader::new(input);

        let token = tokens_iterator.read_next_token();

        let ident = token.unwrap_into_ident(None)?;

        let name = ident.to_string();

        let is_public = if name == "pub" {
            let token = tokens_iterator.read_next_token();
            token.unwrap_into_ident(Some("struct"))?;
            true
        } else {
            false
        };

        let name_ident = tokens_iterator.read_next_token().unwrap_into_ident(None)?;

        let mut token_iterator = tokens_iterator
            .read_next_token()
            .unwrap_into_group(Delimiter::Parenthesis.into())?;

        let type_ident = token_iterator.read_next_token().unwrap_into_ident(None)?;

        Ok(Self {
            is_public,
            name_ident,
            type_ident,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    #[test]
    fn test() {
        let tokens = TokenStream::from_str("pub struct EmailField(String);").unwrap();
        let value = SingleValueTupleStruct::new(tokens).unwrap();

        assert_eq!(value.is_public, true);
        assert_eq!(value.name_ident.to_string(), "EmailField");
        assert_eq!(value.type_ident.to_string(), "String");
    }
}
