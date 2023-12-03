use crate::{StructProperty, TypeName};

pub struct StructureSchema<'s> {
    properties: Vec<StructProperty<'s>>,
    pub name: TypeName<'s>,
}

impl<'s> StructureSchema<'s> {
    pub fn new(data: &'s syn::DeriveInput) -> Self {
        let properties = StructProperty::read(&data).unwrap();
        Self {
            properties,
            name: TypeName::new(data),
        }
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
        if let Some(generic) = self.name.generics {
            let name_ident = self.name.struct_name;
            quote::quote! {
                impl #generic #name_ident #generic{
                    #content
                }
            }
        } else {
            let name_ident = self.name.struct_name;
            quote::quote! {
                impl #name_ident{
                    #content
                }
            }
        }
    }

    pub fn render_try_into_implementation(
        &self,
        from_struct: proc_macro2::TokenStream,
        error_type: proc_macro2::TokenStream,
        content: impl Fn() -> proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let name_ident = self.name.struct_name;
        let content = content();

        let content = quote::quote! {
            type Error = #error_type;

            fn try_into(self) -> Result<ActionMethod, Self::Error> {
                #content
            }
        };

        if let Some(generic) = self.name.generics {
            quote::quote! {
                impl #generic #name_ident #generic{
                    #content
                }
            }
        } else {
            let name_ident = self.name.struct_name;
            quote::quote! {
                impl TryInto<#name_ident> for #from_struct {
                    #content
                }
            }
        }
    }
}
