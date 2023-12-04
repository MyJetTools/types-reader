use crate::{type_name::TypeName, StructProperty};

pub struct StructureSchema<'s> {
    properties: Vec<StructProperty<'s>>,
    pub name: TypeName,
}

impl<'s> StructureSchema<'s> {
    pub fn new(data: &'s syn::DeriveInput) -> Result<Self, syn::Error> {
        let properties = StructProperty::read(&data).unwrap();
        let result = Self {
            properties,
            name: TypeName::from_derive_input(data)?,
        };

        Ok(result)
    }

    pub fn remove(&'s mut self, name: &str) -> Option<StructProperty<'s>> {
        let mut index = None;

        for i in 0..self.properties.len() {
            if self.properties[i].name == name {
                index = Some(i);
                break;
            }
        }

        let index = index?;

        let result = self.properties.remove(index);
        Some(result)
    }

    pub fn get_all(&'s self) -> &'s [StructProperty<'s>] {
        &self.properties
    }

    pub fn render_implement(
        &self,
        content: impl Fn() -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let content = content();

        let generic_after_impl = self.name.get_generic_token_stream_after_impl();

        let name_ident = self.name.to_token_stream();

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
        let mut generic_after_impl = self.name.get_generic_token_stream_after_impl();
        let reference = if from_reference {
            if let Some(life_time) = self.name.get_first_life_time() {
                let life_time_token_stream = life_time.to_token_stream();
                quote::quote!(& #life_time_token_stream)
            } else {
                generic_after_impl = quote::quote!(<'s>);
                quote::quote!(&'s)
            }
        } else {
            quote::quote!()
        };

        let name_ident = self.name.to_token_stream();

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
