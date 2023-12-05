use crate::{
    GenericsArrayToken, LifeTimeToken, PeekedToken, ReferenceToken, TokensReader, TokensTreeExt,
};

pub struct TypeName {
    reference: Option<ReferenceToken>,
    path: Vec<syn::Ident>,
    name: syn::Ident,
    generics: Option<GenericsArrayToken>,
}

impl TypeName {
    pub fn from_token_stream(token_stream: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
        let mut tokens_reader = TokensReader::new(token_stream);

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
            path: Vec::new(),
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
        let mut path = Vec::new();

        for path_step in &self.path {
            path.push(quote::quote!(#path_step::))
        }

        let name = &self.name;

        let generics = if let Some(generics) = &self.generics {
            generics.to_token_stream()
        } else {
            quote::quote! {}
        };

        let reference = match &self.reference {
            Some(reference) => reference.to_token_stream(),
            None => {
                return quote::quote! {#(#path)* #name #generics};
            }
        };

        let name = &self.name;

        quote::quote! {
            #reference #(#path)* #name #generics
        }
    }

    pub fn render_implement(
        &self,
        content: impl Fn() -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let content = content();

        let generic_after_impl = self.get_generic_token_stream_after_impl();

        let name_ident = self.to_token_stream();

        quote::quote! {
            impl #generic_after_impl #name_ident{
                #content
            }
        }
    }

    pub fn render_try_into_implementation(
        &self,
        from_reference: bool,
        from_struct: proc_macro2::TokenStream,
        error_type: proc_macro2::TokenStream,
        content: impl Fn() -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let mut generic_after_impl = self.get_generic_token_stream_after_impl();
        let reference = if from_reference {
            if let Some(life_time) = self.get_first_life_time() {
                let life_time_token_stream = life_time.to_token_stream();
                quote::quote!(& #life_time_token_stream)
            } else {
                generic_after_impl = quote::quote!(<'s>);
                quote::quote!(&'s)
            }
        } else {
            quote::quote!()
        };

        let name_ident = self.to_token_stream();

        let content = content();

        let content = quote::quote! {
            type Error = #error_type;

            fn try_into(self) -> Result<#name_ident, Self::Error> {
                #content
            }
        };

        quote::quote! {
            impl #generic_after_impl TryInto<#name_ident> for #reference #from_struct {
                #content
            }
        }
    }
}

impl TryFrom<proc_macro2::TokenStream> for TypeName {
    type Error = syn::Error;

    fn try_from(value: proc_macro2::TokenStream) -> Result<Self, Self::Error> {
        Self::from_token_stream(value)
    }
}

fn read_name_with_generics(
    tokens_reader: &mut TokensReader,
    reference: Option<ReferenceToken>,
) -> Result<TypeName, syn::Error> {
    let mut vec_of_ident = read_vec_of_ident(tokens_reader)?;

    if vec_of_ident.len() == 0 {
        return Err(tokens_reader.throw_error("Expected struct name"));
    }

    let name = vec_of_ident.remove(vec_of_ident.len() - 1);

    let next_token = tokens_reader.try_peek_next_token();

    if next_token.is_none() {
        return Ok(TypeName {
            reference,
            name,
            generics: None,
            path: vec_of_ident,
        });
    }

    let next_token = next_token.unwrap().unwrap_as_punct_char();

    if let Some(next_token) = next_token {
        if next_token == '<' {
            let generics = GenericsArrayToken::new(tokens_reader)?;
            return Ok(TypeName {
                reference,
                path: vec_of_ident,
                name,
                generics: Some(generics),
            });
        }
    }

    Ok(TypeName {
        reference,
        path: vec_of_ident,
        name,
        generics: None,
    })
}

fn read_vec_of_ident(tokens_reader: &mut TokensReader) -> Result<Vec<syn::Ident>, syn::Error> {
    let mut result = Vec::new();
    while let Some(next_token) = tokens_reader.try_peek_next_token() {
        match next_token {
            PeekedToken::Punct(c) => {
                if c == '<' {
                    break;
                } else {
                    tokens_reader.try_get_next_token().unwrap();
                }
            }
            PeekedToken::Ident => {
                let ident = tokens_reader.try_get_next_token().unwrap();
                result.push(ident.unwrap_as_ident()?);
            }
            PeekedToken::Group(_) => {
                let next_token = tokens_reader.try_get_next_token().unwrap();
                return next_token.throw_error("Expected ident or punct");
            }
            PeekedToken::Literal => {
                let next_token = tokens_reader.try_get_next_token().unwrap();
                return next_token.throw_error("Expected ident or punct");
            }
        }
    }

    Ok(result)
}

impl<'s> TryInto<TypeName> for &syn::DeriveInput {
    type Error = syn::Error;

    fn try_into(self) -> Result<TypeName, Self::Error> {
        TypeName::from_derive_input(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_struct_name_with_name_space() {
        let src = proc_macro2::TokenStream::from_str("my_postgres::sql_select::FromDbRow").unwrap();
        let struct_name = TypeName::from_token_stream(src).unwrap();
        assert_eq!(
            "my_postgres :: sql_select :: FromDbRow",
            struct_name.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_struct_name_with_name_space_and_lifetime_as_generic() {
        let src =
            proc_macro2::TokenStream::from_str("my_postgres::sql_select::FromDbRow<'s>").unwrap();
        let struct_name = TypeName::from_token_stream(src).unwrap();
        assert_eq!(
            "my_postgres :: sql_select :: FromDbRow < 's >",
            struct_name.to_token_stream().to_string()
        );
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

    #[test]
    fn test_struct_name_as_a_reference() {
        let src = proc_macro2::TokenStream::from_str("&'s MyName").unwrap();
        let struct_name = TypeName::from_token_stream(src).unwrap();
        assert_eq!("& 's MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name() {
        let src = proc_macro2::TokenStream::from_str("MyName").unwrap();
        let struct_name = TypeName::from_token_stream(src).unwrap();
        assert_eq!("MyName", struct_name.to_token_stream().to_string());
    }

    #[test]
    fn test_struct_name_with_generic() {
        let src = proc_macro2::TokenStream::from_str("MyName<'s>").unwrap();

        let struct_name: TypeName = src.try_into().unwrap();

        assert_eq!("MyName < 's >", struct_name.to_token_stream().to_string());
    }
}
