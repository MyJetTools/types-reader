use std::collections::HashMap;

use macros_utils::AttributeParams;

use crate::PropertyType;

pub struct StructProperty<'s> {
    pub name: String,
    pub ty: PropertyType<'s>,
    pub field: &'s syn::Field,
    pub attrs: HashMap<String, Option<AttributeParams>>,
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
                name,
                field,
                ty: PropertyType::new(field),
                attrs,
            })
        }

        result
    }

    pub fn get_field_name_ident(&self) -> &syn::Ident {
        &self.field.ident.as_ref().unwrap()
    }
    pub fn get_syn_type(&self) -> &syn::Type {
        &self.field.ty
    }
}
