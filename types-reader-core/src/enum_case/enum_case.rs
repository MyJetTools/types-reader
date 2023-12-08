use crate::{attributes::Attributes, EnumModel, MacrosAttribute, TokensObject};

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

    pub fn get_attributes<
        TResult: MacrosAttribute + TryFrom<&'s TokensObject, Error = syn::Error>,
    >(
        &'s self,
    ) -> Result<Option<Vec<TResult>>, syn::Error> {
        let attrs = self.attrs.get_attrs(TResult::NAME)?;
        if attrs.len() == 0 {
            return Ok(None);
        }

        let mut result = Vec::with_capacity(attrs.len());

        for attr in attrs {
            let itm = TResult::try_from(attr)?;
            result.push(itm);
        }

        Ok(Some(result))
    }

    pub fn try_get_attributes<
        TResult: MacrosAttribute + TryFrom<&'s TokensObject, Error = syn::Error>,
    >(
        &'s self,
    ) -> Result<Option<Vec<TResult>>, syn::Error> {
        let attrs = self.attrs.try_get_attrs(TResult::NAME);

        if attrs.is_none() {
            return Ok(None);
        }

        let attrs = attrs.unwrap();

        let mut result = Vec::with_capacity(attrs.len());

        for attr in attrs {
            let itm = TResult::try_from(attr)?;
            result.push(itm);
        }

        Ok(Some(result))
    }
}
