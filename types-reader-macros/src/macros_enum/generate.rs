use proc_macro::TokenStream;
use types_reader_core::EnumCase;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name_ident = &ast.ident;

    let src_fields = EnumCase::read(&ast)?;

    let mut try_from_str_cases = Vec::new();

    let mut supported_cases = String::new();

    let mut enum_cases_with_model = Vec::new();

    let mut has_default_case = None;

    let mut as_str_cases = Vec::with_capacity(src_fields.len());

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

        if try_from_str_cases.len() > 0 {
            supported_cases.push_str(",");
        }

        supported_cases.push('\'');
        supported_cases.push_str(case_as_str);

        try_from_str_cases.push(quote::quote! {
            if value == #case_as_str{
                return Some(Self::#case_ident);
            }
        });
        supported_cases.push('\'');

        as_str_cases.push(quote::quote!(#name_ident::#case_ident => #case_as_str,));
    }

    let mut generated_model_cases = Vec::with_capacity(enum_cases_with_model.len());

    let mut has_vec_case = false;

    for enum_case in enum_cases_with_model {
        let case_ident = enum_case.get_name_ident();

        let model = enum_case.model.as_ref().unwrap();
        let ident = model.get_name_ident();

        let ident_as_string = ident.to_string();

        if ident_as_string == "Vec" {
            has_vec_case = true;
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

    let try_into_error = quote::quote! {
        let err = self.throw_error(
            format!(
                "Unsupported value: {}. Supported values are: {}",
                value, #supported_cases
            )
            .as_str(),
           );
           Err(err)
    };

    let as_str_impl = if has_vec_case || as_str_cases.len() == 0 {
        quote::quote!()
    } else {
        quote::quote! {
            pub fn as_str(&self) -> &str {
                match self {
                    #( #as_str_cases )*
                }
            }
        }
    };

    let result = quote::quote! {


        impl #name_ident {
            pub fn try_from_str(value: &str) -> Option<Self> {
                #( #try_from_str_cases )*
                None
            }

            #as_str_impl
        }


        impl<'s> TryInto<#name_ident> for &'s types_reader::ObjectValue{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                let value = self.as_string()?.as_str();

                if let Some(value) = #name_ident::try_from_str(value){
                    return Ok(value);
                }

                #try_into_error

            }
        }

        impl<'s> TryInto<#name_ident> for &'s types_reader::OptionalObjectValue{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                let value = self.as_string()?.as_str();

                if let Some(value) = #name_ident::try_from_str(value){
                    return Ok(value);
                }

                #try_into_error

            }
        }

        impl<'s> TryInto<#name_ident> for &'s types_reader::TokensObject{
            type Error = syn::Error;
            fn try_into(self) -> Result<#name_ident, Self::Error> {
                #( #generated_model_cases )*
                let value = self.unwrap_as_value()?;
                Ok(value.try_into()?)
            }
        }

        impl<'s> TryInto<#name_ident> for &'s dyn types_reader::AnyValueAsStr<'s> {
            type Error = syn::Error;

            fn try_into(self) -> Result<#name_ident, Self::Error> {
                let value = self.as_str()?;

                if let Some(value) = #name_ident::try_from_str(value) {
                    return Ok(value);
                }

                #try_into_error
            }
        }

        #impl_default

    };

    Ok(result.into())
}
