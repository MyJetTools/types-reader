use std::collections::HashMap;

use macros_utils::AttributeParams;

use crate::EnumModel;

pub struct EnumCase<'s> {
    pub attrs: HashMap<String, Option<AttributeParams>>,
    pub variant: &'s syn::Variant,
    pub name: String,
    pub model: Option<EnumModel>,
}

impl<'s> EnumCase<'s> {
    pub fn read(ast: &'s syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
            for variant in variants {
                let name = variant.ident.to_string();

                let model = EnumModel::new(variant);
                result.push(EnumCase {
                    attrs: crate::attributes::parse(&variant.attrs),
                    variant,
                    name,
                    model,
                });
            }
        } else {
            panic!("Enum Only")
        };

        result
    }

    pub fn get_name_ident(&self) -> &syn::Ident {
        &self.variant.ident
    }
}
