use std::collections::HashMap;

use crate::attribute_params::{AttributeParams, ParamValue};

pub struct Attributes<'s> {
    attrs: HashMap<String, AttributeParams<'s>>,
}

impl<'s> Attributes<'s> {
    pub fn new(src: &'s [syn::Attribute]) -> Result<Self, syn::Error> {
        let mut attrs = HashMap::new();

        for attr in src {
            let attr = AttributeParams::new(attr)?;
            attrs.insert(attr.get_id_token().to_string(), attr);
        }

        Ok(Self { attrs })
    }

    pub fn get_named_param(&'s self, attr_name: &str, param_name: &str) -> Option<ParamValue> {
        let attr = self.attrs.get(attr_name)?;
        attr.get_named_param(param_name)
    }

    pub fn get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Option<ParamValue> {
        let attr = self.attrs.get(attr_name)?;
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
}
