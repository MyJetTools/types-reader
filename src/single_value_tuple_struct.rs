use proc_macro2::{Delimiter, Ident};

use crate::TokensReader;

pub struct SingleValueTupleStruct {
    pub is_public: bool,
    pub name_ident: Ident,
    pub is_type_public: bool,
    pub type_ident: Ident,
}

impl SingleValueTupleStruct {
    pub fn new(input: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
        let mut token_reader = TokensReader::new(input);

        let token = token_reader.read_next_token();

        let ident = token.unwrap_into_ident(None)?;

        let name = ident.to_string();

        let is_public = if name == "pub" {
            let token = token_reader.read_next_token();
            token.unwrap_into_ident(Some("struct"))?;
            true
        } else {
            false
        };

        let name_ident = token_reader.read_next_token().unwrap_into_ident(None)?;

        let mut token_iterator = token_reader
            .read_next_token()
            .unwrap_into_group(Delimiter::Parenthesis.into())?;

        let mut is_type_public = false;

        let mut type_ident = token_iterator.read_next_token().unwrap_into_ident(None)?;

        if type_ident.to_string() == "pub" {
            is_type_public = true;
            type_ident = token_iterator.read_next_token().unwrap_into_ident(None)?;
        }

        Ok(Self {
            is_public,
            name_ident,
            type_ident,
            is_type_public,
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
        assert_eq!(value.is_type_public, false);
        assert_eq!(value.name_ident.to_string(), "EmailField");
        assert_eq!(value.type_ident.to_string(), "String");
    }

    #[test]
    fn test_private_struct() {
        let tokens = TokenStream::from_str("struct EmailField(String);").unwrap();
        let value = SingleValueTupleStruct::new(tokens).unwrap();

        assert_eq!(value.is_public, false);
        assert_eq!(value.is_type_public, false);
        assert_eq!(value.name_ident.to_string(), "EmailField");
        assert_eq!(value.type_ident.to_string(), "String");
    }

    #[test]
    fn test_public_struct_with_pub_access() {
        let tokens = TokenStream::from_str("pub struct EmailField(pub String);").unwrap();
        let value = SingleValueTupleStruct::new(tokens).unwrap();

        assert_eq!(value.is_public, true);
        assert_eq!(value.is_type_public, true);
        assert_eq!(value.name_ident.to_string(), "EmailField");
        assert_eq!(value.type_ident.to_string(), "String");
    }
}
