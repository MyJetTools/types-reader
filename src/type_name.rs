use proc_macro2::TokenStream;
use quote::quote;
pub struct TypeName<'s> {
    pub struct_name: &'s syn::Ident,
    pub generics: Option<&'s syn::Generics>,
}

impl<'s> TypeName<'s> {
    pub fn new(ast: &'s syn::DeriveInput) -> Self {
        if ast.generics.lifetimes().count() > 0 {
            let generics = &ast.generics;
            Self {
                struct_name: &ast.ident,
                generics: Some(generics),
            }
        } else {
            Self {
                struct_name: &ast.ident,
                generics: None,
            }
        }
    }

    pub fn get_type_name(&self) -> TokenStream {
        let ident = self.struct_name;
        if let Some(generics) = self.generics {
            quote!(#ident #generics)
        } else {
            quote!(#ident)
        }
    }

    pub fn get_default_lifetime_generic(&self) -> TokenStream {
        if let Some(generics) = self.generics {
            quote!(#generics)
        } else {
            quote!(<'s>)
        }
    }
}
