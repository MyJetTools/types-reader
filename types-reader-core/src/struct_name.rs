use crate::{GenericsArrayToken, ReferenceToken, TokensReader};

pub struct StructName {
    reference: Option<ReferenceToken>,
    name: syn::Ident,
    generics: Option<GenericsArrayToken>,
}

impl StructName {
    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let name = &self.name;

        let generics = if let Some(generics) = &self.generics {
            generics.to_token_stream()
        } else {
            quote::quote! {}
        };

        let reference = match &self.reference {
            Some(reference) => reference.to_token_stream(),
            None => {
                return quote::quote! {#name #generics};
            }
        };

        let name = &self.name;

        quote::quote! {
            #reference #name #generics
        }
    }
}

impl TryFrom<proc_macro2::TokenStream> for StructName {
    type Error = syn::Error;

    fn try_from(value: proc_macro2::TokenStream) -> Result<Self, Self::Error> {
        let mut tokens_reader = TokensReader::new(value);

        let next_peeked_token = tokens_reader.peek_next_token("Expected struct name")?;

        if next_peeked_token.is_ident() {
            return read_name_with_generics(&mut tokens_reader, None);
        }

        if !next_peeked_token.is_punct() {
            return Err(tokens_reader.throw_error("Expected struct name or reference"));
        }

        if next_peeked_token.unwrap_as_punct_char().unwrap() == '&' {
            let reference = ReferenceToken::new(&mut tokens_reader)?;
            return read_name_with_generics(&mut tokens_reader, Some(reference));
        }

        Err(tokens_reader.throw_error("Expected struct name or reference"))
    }
}

fn read_name_with_generics(
    tokens_reader: &mut TokensReader,
    reference: Option<ReferenceToken>,
) -> Result<StructName, syn::Error> {
    let next_token = tokens_reader.read_next_token()?;

    let name = next_token.unwrap_into_ident(None)?;

    let next_token = tokens_reader.try_peek_next_token();

    if next_token.is_none() {
        return Ok(StructName {
            reference,
            name,
            generics: None,
        });
    }

    let next_token = next_token.unwrap().unwrap_as_punct_char();

    if let Some(next_token) = next_token {
        if next_token == '<' {
            let generics = GenericsArrayToken::new(tokens_reader)?;
            return Ok(StructName {
                reference,
                name,
                generics: Some(generics),
            });
        }
    }

    Ok(StructName {
        reference,
        name,
        generics: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_struct_name_as_a_reference() {
        let src = proc_macro2::TokenStream::from_str("&'s MyName").unwrap();

        let struct_name: StructName = src.try_into().unwrap();
        assert_eq!("& 's MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name() {
        let src = proc_macro2::TokenStream::from_str("MyName").unwrap();

        let struct_name: StructName = src.try_into().unwrap();

        assert_eq!("MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name_with_generic() {
        let src = proc_macro2::TokenStream::from_str("MyName<'s>").unwrap();

        let struct_name: StructName = src.try_into().unwrap();

        assert_eq!("MyName < 's >", struct_name.to_token_stream().to_string());
    }
}
