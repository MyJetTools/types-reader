use std::collections::HashMap;

use crate::attribute_params::{AttributeParams, ParamValue};

pub struct Attributes<'s> {
    attrs: HashMap<String, Vec<AttributeParams>>,
    root: &'s syn::DeriveInput,
}

impl<'s> Attributes<'s> {
    pub fn new(root: &'s syn::DeriveInput, src: &'s [syn::Attribute]) -> Result<Self, syn::Error> {
        let mut attrs = HashMap::new();

        for attr in src {
            let attr = AttributeParams::new(attr)?;
            let name = attr.get_name();
            if !attrs.contains_key(&name) {
                attrs.insert(attr.get_name(), Vec::new());
            }
            attrs.get_mut(name.as_str()).unwrap().push(attr);
        }

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

        Ok(attr.unwrap().get(0).unwrap())
    }

    pub fn get_attrs(&'s self, attr_name: &str) -> Result<&'s Vec<AttributeParams>, syn::Error> {
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
        let result = self.attrs.contains_key(name);

        result
    }

    pub fn has_attr_debug(&self, field_name: &str, name: &str) -> bool {
        let result = self.attrs.contains_key(name);

        println!(
            "Field: {}. Looking for attr {} is in attrs: {:?}. Result: {}",
            field_name,
            name,
            self.attrs.keys(),
            result
        );

        result
    }

    pub fn has_attr_and_param(&self, attr_name: &str, param_name: &str) -> bool {
        if let Some(attr) = self.attrs.get(attr_name) {
            return attr.first().unwrap().has_param(param_name);
        }

        false
    }

    pub fn remove(&'s mut self, name: &str) -> Option<Vec<AttributeParams>> {
        self.attrs.remove(name)
    }

    pub fn get_attr_names(
        &'s self,
    ) -> std::collections::hash_map::Keys<String, Vec<AttributeParams>> {
        self.attrs.keys()
    }
}
