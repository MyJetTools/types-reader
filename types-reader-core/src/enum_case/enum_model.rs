pub struct EnumModel<'s> {
    ident: &'s syn::Ident,
}

impl<'s> EnumModel<'s> {
    pub fn new(variant: &'s syn::FieldsUnnamed) -> Result<Self, syn::Error> {
        for field in &variant.unnamed {
            match &field.ty {
                syn::Type::Path(type_path) => {
                    for segment in &type_path.path.segments {
                        return Ok(Self {
                            ident: &segment.ident,
                        });
                    }
                }

                _ => {
                    return Err(syn::Error::new_spanned(
                        variant,
                        format!("Invalid type: {:#?}", field.ty),
                    ));
                }
            }
        }

        return Err(syn::Error::new_spanned(variant, "No model found"));
    }

    pub fn get_name_ident(&'s self) -> &'s syn::Ident {
        &self.ident
    }
}
