use macros_utils::attributes::Attributes;

pub struct EnumCase<'s> {
    pub attrs: Attributes,
    pub variant: &'s syn::Variant,
}

impl<'s> EnumCase<'s> {
    pub fn read(ast: &'s syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
            for variant in variants {
                result.push(EnumCase {
                    attrs: crate::attributes::parse(&variant.attrs),
                    variant,
                });
            }
        } else {
            panic!("Enum Only")
        };

        result
    }

    pub fn get_name(&self) -> String {
        self.variant.ident.to_string()
    }

    pub fn get_name_ident(&self) -> &syn::Ident {
        &self.variant.ident
    }
}
