use crate::{attributes::Attributes, EnumModel};

pub struct EnumCase<'s> {
    pub attrs: Attributes<'s>,
    name_ident: &'s syn::Ident,
    pub model: Option<EnumModel<'s>>,
}

impl<'s> EnumCase<'s> {
    pub fn read(ast: &'s syn::DeriveInput) -> Result<Vec<Self>, syn::Error> {
        let mut result = Vec::new();

        if let syn::Data::Enum(data_enum) = &ast.data {
            for variant in data_enum.variants.iter() {
                match &variant.fields {
                    syn::Fields::Named(_) => {
                        return Err(syn::Error::new_spanned(
                            variant,
                            "Named enum case is not supported",
                        ));
                    }
                    syn::Fields::Unnamed(data) => {
                        let model = EnumModel::new(data)?;
                        result.push(EnumCase {
                            attrs: Attributes::new(ast, &variant.attrs)?,
                            model: Some(model),
                            name_ident: &variant.ident,
                        });
                    }
                    syn::Fields::Unit => {
                        result.push(EnumCase {
                            attrs: Attributes::new(ast, &variant.attrs)?,
                            model: None,
                            name_ident: &variant.ident,
                        });
                    }
                }
            }
        } else {
            panic!("Enum Only")
        };

        Ok(result)
    }

    pub fn get_name_ident(&self) -> &syn::Ident {
        &self.name_ident
    }
}
