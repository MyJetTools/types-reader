use macros_utils::attributes::Attributes;
use proc_macro2::Ident;

use super::PropertyType;

pub struct StructProperty<'s> {
    pub name_ident: &'s Ident,
    pub name: String,
    pub ty: PropertyType,
    pub attrs: Attributes,
}

impl<'s> StructProperty<'s> {
    pub fn read(ast: &'s syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        let fields = if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) = ast.data
        {
            fields
        } else {
            panic!("Struct Only")
        };

        for field in &fields.named {
            let attrs = super::attributes::parse(&field.attrs);

            let name = field.ident.as_ref().unwrap().to_string();

            result.push(Self {
                name_ident: field.ident.as_ref().unwrap(),
                name,
                ty: PropertyType::new(field),
                attrs,
            })
        }

        result
    }
}
