use macros_utils::attributes::Attributes;

use super::PropertyType;

pub struct StructProperty {
    pub name: String,
    pub ty: PropertyType,
    pub attrs: Attributes,
}

impl StructProperty {
    pub fn read(ast: &syn::DeriveInput) -> Vec<Self> {
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

            result.push(Self {
                name: field.ident.as_ref().unwrap().to_string(),
                ty: PropertyType::new(field),
                attrs,
            })
        }

        result
    }
}
