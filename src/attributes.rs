use std::collections::HashMap;

use crate::attribute_params::{AttributeParams, ParamValue};

pub struct Attributes<'s> {
    attrs: HashMap<String, AttributeParams>,
    root: &'s syn::DeriveInput,
}

impl<'s> Attributes<'s> {
    pub fn new(root: &'s syn::DeriveInput, src: &'s [syn::Attribute]) -> Result<Self, syn::Error> {
        let mut attrs = HashMap::new();

        for attr in src {
            let attr = AttributeParams::new(attr)?;
            attrs.insert(attr.get_name(), attr);
        }

        println!("Attributes: {:?}", attrs.keys());

        Ok(Self { root, attrs })
    }

    pub fn get_attr(&'s self, attr_name: &str) -> Result<&'s AttributeParams, syn::Error> {
        let attr = self.attrs.get(attr_name);

        if attr.is_none() {
            return Err(syn::Error::new_spanned(
                self.root,
                format!("Attribute {} not found", attr_name),
            ));
        }

        Ok(attr.unwrap())
    }

    pub fn get_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<ParamValue, syn::Error> {
        let attr = self.get_attr(attr_name)?;
        attr.get_named_param(param_name)
    }

    pub fn get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<ParamValue, syn::Error> {
        let attr = self.get_attr(attr_name)?;

        attr.get_from_single_or_named(param_name)
    }

    pub fn has_attr(&self, name: &str) -> bool {
        self.attrs.contains_key(name)
    }

    pub fn has_attr_and_param(&self, attr_name: &str, param_name: &str) -> bool {
        if let Some(attr) = self.attrs.get(attr_name) {
            return attr.has_param(param_name);
        }

        false
    }

    pub fn remove(&'s mut self, name: &str) -> Option<AttributeParams> {
        self.attrs.remove(name)
    }

    pub fn get_attr_names(&'s self) -> std::collections::hash_map::Keys<String, AttributeParams> {
        self.attrs.keys()
    }
}
