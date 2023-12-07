use crate::{
    attributes::{Attributes, MacrosAttribute},
    PropertyType, TokensObject,
};

pub struct StructProperty<'s> {
    pub name: String,
    pub ty: PropertyType<'s>,
    pub field: &'s syn::Field,
    pub attrs: Attributes<'s>,
}

impl<'s> StructProperty<'s> {
    pub fn read(ast: &'s syn::DeriveInput) -> Result<Vec<Self>, syn::Error> {
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
            let attrs = Attributes::new(ast, &field.attrs)?;

            let name = field.ident.as_ref().unwrap().to_string();

            result.push(Self {
                name,
                field,
                ty: PropertyType::new(field),
                attrs,
            })
        }

        Ok(result)
    }

    pub fn get_field_name_ident(&self) -> &syn::Ident {
        &self.field.ident.as_ref().unwrap()
    }
    pub fn get_syn_type(&self) -> &syn::Type {
        &self.field.ty
    }

    pub fn throw_error<TResult>(&self, message: &str) -> Result<TResult, syn::Error> {
        let err = syn::Error::new_spanned(self.field, message);
        Err(err)
    }

    pub fn try_get_attribute<
        TResult: MacrosAttribute + TryFrom<&'s TokensObject, Error = syn::Error>,
    >(
        &'s self,
    ) -> Result<Option<TResult>, syn::Error> {
        let result = self.attrs.try_get_attr(TResult::NAME);

        if result.is_none() {
            return Ok(None);
        }

        let result = result.unwrap();

        let result = TResult::try_from(result)?;
        Ok(Some(result))
    }

    pub fn get_attribute<
        TResult: MacrosAttribute + TryFrom<&'s TokensObject, Error = syn::Error>,
    >(
        &'s self,
    ) -> Result<TResult, syn::Error> {
        let result = self.attrs.get_attr(TResult::NAME)?;

        let result = TResult::try_from(result)?;
        Ok(result)
    }
}
