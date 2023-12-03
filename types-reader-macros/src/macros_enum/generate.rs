use proc_macro::TokenStream;
use types_reader_core::EnumCase;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name_ident = &ast.ident;

    let src_fields = EnumCase::read(&ast)?;

    let mut cases = Vec::new();

    let mut supported_cases = String::new();

    for src in src_fields {
        let case_ident = src.get_name_ident();
        let case_name = case_ident.to_string().to_uppercase();

        if cases.len() > 0 {
            supported_cases.push_str(",");
        }

        supported_cases.push('\'');
        supported_cases.push_str(case_name.as_str());
        cases.push(quote::quote! {
            if value == #case_name{
                return Ok(ActionMethod::#case_ident);
            }
        });
        supported_cases.push('\'');
    }

    let result = quote::quote! {


        impl<'s> TryInto<#name_ident> for &'s types_reader::ObjectValue{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                let value = self.as_string()?.as_str().to_uppercase();

                #( #cases )*

                panic!("Unsupported value: {}. Supported values are: {}", value, #supported_cases);
            }
        }

    };

    Ok(result.into())
}
