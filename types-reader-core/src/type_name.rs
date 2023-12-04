use crate::{GenericsArrayToken, LifeTimeToken, ReferenceToken, TokensReader};

pub struct TypeName {
    reference: Option<ReferenceToken>,
    name: syn::Ident,
    generics: Option<GenericsArrayToken>,
}

impl TypeName {
    pub fn from_derive_input(ast: &syn::DeriveInput) -> Result<Self, syn::Error> {
        let name = &ast.ident;

        let generics = if ast.generics.lifetimes().count() > 0 {
            let generics = &ast.generics;
            let mut tokens_reader = TokensReader::new(quote::quote!(#generics));
            Some(GenericsArrayToken::new(&mut tokens_reader)?)
        } else {
            None
        };

        Ok(Self {
            reference: None,
            name: name.clone(),
            generics,
        })
    }

    pub fn has_life_time(&self) -> bool {
        self.generics.is_some()
    }

    pub fn has_generics(&self) -> bool {
        self.generics.is_some()
    }

    pub fn get_generic_token_stream_after_impl(&self) -> proc_macro2::TokenStream {
        if let Some(generics) = &self.generics {
            generics.to_token_stream()
        } else {
            quote::quote! {}
        }
    }

    pub fn get_first_life_time(&self) -> Option<&LifeTimeToken> {
        if let Some(generics) = &self.generics {
            generics.get_first_life_time()
        } else {
            None
        }
    }

    pub fn get_name_ident(&self) -> &syn::Ident {
        &self.name
    }

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

impl TryFrom<proc_macro2::TokenStream> for TypeName {
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
) -> Result<TypeName, syn::Error> {
    let next_token = tokens_reader.read_next_token()?;

    let name = next_token.unwrap_into_ident(None)?;

    let next_token = tokens_reader.try_peek_next_token();

    if next_token.is_none() {
        return Ok(TypeName {
            reference,
            name,
            generics: None,
        });
    }

    let next_token = next_token.unwrap().unwrap_as_punct_char();

    if let Some(next_token) = next_token {
        if next_token == '<' {
            let generics = GenericsArrayToken::new(tokens_reader)?;
            return Ok(TypeName {
                reference,
                name,
                generics: Some(generics),
            });
        }
    }

    Ok(TypeName {
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

        let struct_name: TypeName = src.try_into().unwrap();
        assert_eq!("& 's MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name() {
        let src = proc_macro2::TokenStream::from_str("MyName").unwrap();

        let struct_name: TypeName = src.try_into().unwrap();

        assert_eq!("MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name_with_generic() {
        let src = proc_macro2::TokenStream::from_str("MyName<'s>").unwrap();

        let struct_name: TypeName = src.try_into().unwrap();

        assert_eq!("MyName < 's >", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_from_derive_input_ident() {
        let src = proc_macro2::TokenStream::from_str(
            r#"pub struct HttpActionResult{
            pub status_code: u16,
            pub description: &'s str,
            }
        "#,
        )
        .unwrap();

        let derive_input = syn::parse2::<syn::DeriveInput>(src).unwrap();

        let struct_name = TypeName::from_derive_input(&derive_input).unwrap();

        assert_eq!(
            "HttpActionResult",
            struct_name.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_from_derive_input_ident_with_life_time_generic() {
        let src = proc_macro2::TokenStream::from_str(
            r#"pub struct HttpActionResult<'s>{
            pub status_code: u16,
            pub description: &'s str,
            }"#,
        )
        .unwrap();

        let derive_input = syn::parse2::<syn::DeriveInput>(src).unwrap();

        let struct_name = TypeName::from_derive_input(&derive_input).unwrap();

        assert_eq!(
            "HttpActionResult < 's >",
            struct_name.to_token_stream().to_string()
        );
    }
}
