use proc_macro::TokenStream;
use types_reader_core::EnumCase;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name_ident = &ast.ident;

    let src_fields = EnumCase::read(&ast)?;

    let mut try_into_cases = Vec::new();
    let mut into_cases = Vec::new();

    let mut supported_cases = String::new();

    let mut enum_cases_with_model = Vec::new();

    let mut has_default_case = None;

    for src in &src_fields {
        if super::utils::has_default_attribute(&src) {
            has_default_case = Some(src.get_name_ident());
        }

        if src.model.is_some() {
            enum_cases_with_model.push(src);
            continue;
        };

        let case_ident = src.get_name_ident();
        let case_as_str = super::utils::get_enum_str_value(&src)?;
        let case_as_str = case_as_str.as_str();

        if try_into_cases.len() > 0 {
            supported_cases.push_str(",");
        }

        supported_cases.push('\'');
        supported_cases.push_str(case_as_str);
        try_into_cases.push(quote::quote! {
            if value == #case_as_str{
                return Ok(#name_ident::#case_ident);
            }
        });

        into_cases.push(quote::quote! {
            if self == #case_as_str{
                return #name_ident::#case_ident;
            }
        });
        supported_cases.push('\'');
    }

    let mut generated_model_cases = Vec::with_capacity(enum_cases_with_model.len());

    for enum_case in enum_cases_with_model {
        let case_ident = enum_case.get_name_ident();

        let model = enum_case.model.as_ref().unwrap();
        let ident = model.get_name_ident();

        let ident_as_string = ident.to_string();

        if ident_as_string == "Vec" {
            generated_model_cases.push(quote::quote! {
                if let Some(src) = self.try_get_vec(){
                    let mut result = Vec::with_capacity(src.len());

                    for itm in src{
                        result.push(itm.try_into()?);
                    }

                    return Ok(#name_ident::#case_ident(result));
                }
            });
        } else {
            generated_model_cases.push(quote::quote! {
                if self.is_object() {
                    return Ok(#name_ident::#case_ident(self.try_into()?));
                }
            });
        }
    }

    let impl_default = if let Some(default_case) = has_default_case {
        quote::quote! {
            impl Default for #name_ident{
                fn default() -> Self {
                    Self::#default_case
                }
            }
        }
    } else {
        quote::quote!()
    };

    let result = quote::quote! {


        impl<'s> TryInto<#name_ident> for &'s types_reader::ObjectValue{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                let value = self.as_string()?.as_str();

                #( #try_into_cases )*

                panic!("Unsupported value: {}. Supported values are: {}", value, #supported_cases);
            }
        }

        impl<'s> TryInto<#name_ident> for &'s types_reader::TokensObject{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                #( #generated_model_cases )*
                let value = self.get_value()?;
                Ok(value.try_into()?)
            }
        }

        impl<'s> Into<#name_ident> for &'s str{
            fn into(self) -> #name_ident {
                #( #into_cases )*
                panic!("Unsupported value: {}. Supported values are: {}", self, #supported_cases);
            }
        }

        #impl_default

    };

    Ok(result.into())
}
