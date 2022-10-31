use macros_utils::attributes::Attributes;

pub struct EnumCase {
    pub attrs: Attributes,
    pub name: String,
}

impl EnumCase {
    pub fn read(ast: &syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
            for varian in variants {
                result.push(EnumCase {
                    name: varian.ident.to_string(),
                    attrs: crate::attributes::parse(&varian.attrs),
                });
            }
        } else {
            panic!("Enum Only")
        };

        result
    }
}
