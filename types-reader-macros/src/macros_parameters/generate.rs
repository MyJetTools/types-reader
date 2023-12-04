use std::str::FromStr;

use proc_macro::TokenStream;
use types_reader_core::{PropertyType, StructureSchema};

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let structure_schema = StructureSchema::new(&ast)?;

    let mut reading_props = Vec::new();

    for property in structure_schema.get_all() {
        let prop_ident = property.get_field_name_ident();

        let prop_name = prop_ident.to_string();

        let ident_is_allowed = super::utils::is_ident_allowed(property);

        let (fn_name, opt_fn_name) = if super::utils::is_default(property) {
            (
                "get_from_single_or_named",
                "try_get_value_from_single_or_named",
            )
        } else {
            ("get_named_param", "try_get_named_param")
        };

        let fn_name = proc_macro2::TokenStream::from_str(fn_name).unwrap();
        let opt_fn_name = proc_macro2::TokenStream::from_str(opt_fn_name).unwrap();

        if property.ty.is_vec() {
            reading_props.push(quote::quote!(#prop_ident: {
                let mut result = Vec::new();
                let items = self.#fn_name(#prop_name)?.get_vec()?;

                for item in items {
                    result.push(item.try_into()?);
                }

                result
            }));
        } else if let PropertyType::OptionOf(sub_ty) = &property.ty {
            if sub_ty.is_vec() {
                reading_props.push(
                    quote::quote!(#prop_ident: if let Some(value) = self.#opt_fn_name(#prop_name){
                    let items = value.get_vec()?;
                    let mut result = Vec::new();
                    for item in items {
                        result.push(item.try_into()?);
                    }
    
                    Some(result)
                }else{
                    None
                }, ),
                );
            } else {
                if ident_is_allowed {
                    reading_props.push(
                        quote::quote!(#prop_ident: if let Some(value) = self.#opt_fn_name(#prop_name){
                        Some(value.get_value()?.get_any_value_as_str()?.into())
                    }else{
                        None
                    }, ),
                    );
                } else {
                    reading_props.push(
                        quote::quote!(#prop_ident: if let Some(value) = self.#opt_fn_name(#prop_name){
                        Some(value.try_into()?)
                    }else{
                        None
                    }, ),
                    );
                }
            }
        } else if let PropertyType::RefTo { ty, lifetime: _ } = &property.ty {
            if ty.as_str().as_str() == "TokensObject" {
                reading_props.push(quote::quote! {
                    #prop_ident: self.#fn_name(#prop_name)?,
                });
            } else {
                reading_props.push(quote::quote! {
                    #prop_ident: self.#fn_name(#prop_name)?.try_into()?,
                });
            }
        } else {
            if ident_is_allowed {
                reading_props.push(quote::quote! {
                    #prop_ident: self.#fn_name(#prop_name)?.get_value()?.get_any_value_as_str()?.into(),
                });
            } else {
                reading_props.push(quote::quote! {
                    #prop_ident: self.#fn_name(#prop_name)?.try_into()?,
                });
            }
        }
    }

    let name_ident = structure_schema.name.get_name_ident();

    let from_tokens_object = structure_schema.render_try_into_implementation(
        true,
        quote::quote!(types_reader::TokensObject),
        quote::quote!(syn::Error),
        || {
            quote::quote! {
                    let result = #name_ident{
                        #( #reading_props )*
                    };
                    Ok(result)
            }
        },
    );

    Ok(from_tokens_object.into())
}
