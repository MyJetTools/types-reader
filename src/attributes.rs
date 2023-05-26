use std::collections::HashMap;

use quote::ToTokens;

use crate::{ParamValue, ParamsList};

pub struct Attributes<'s> {
    attrs: HashMap<String, Vec<ParamsList>>,
    root: &'s syn::DeriveInput,
}

impl<'s> Attributes<'s> {
    pub fn new(root: &'s syn::DeriveInput, src: &'s [syn::Attribute]) -> Result<Self, syn::Error> {
        let mut attrs = HashMap::new();

        for attr in src {
            let token: proc_macro2::TokenStream = attr.to_token_stream();

            let mut tokens = token.into_iter();

            let ident = tokens.next().unwrap();

            let name = ident.to_string();

            let braces_token = tokens.next().unwrap();

            match braces_token {
                proc_macro2::TokenTree::Group(value) => {
                    let token = value.to_token_stream();

                    let attr = ParamsList::new(token)?;

                    if !attrs.contains_key(&name) {
                        attrs.insert(name.clone(), Vec::new());
                    }
                    attrs.get_mut(name.as_str()).unwrap().push(attr);
                }
                proc_macro2::TokenTree::Ident(value) => {
                    panic!("Somehow we got Ident here: {}", value.to_string());
                }
                proc_macro2::TokenTree::Punct(value) => {
                    panic!("Somehow we got Punct here: {}", value.to_string());
                }
                proc_macro2::TokenTree::Literal(value) => {
                    panic!("Somehow we got Literal here: {}", value.to_string());
                }
            }
        }

        Ok(Self { root, attrs })
    }

    pub fn get_attr(&'s self, attr_name: &str) -> Result<&'s ParamsList, syn::Error> {
        let attr = self.attrs.get(attr_name);

        if attr.is_none() {
            return Err(syn::Error::new_spanned(
                self.root,
                format!("Attribute {} not found", attr_name),
            ));
        }

        Ok(attr.unwrap().get(0).unwrap())
    }

    pub fn try_get_attr(&'s self, attr_name: &str) -> Option<&'s ParamsList> {
        let attr = self.attrs.get(attr_name)?;

        Some(attr.get(0).unwrap())
    }

    pub fn get_attrs(&'s self, attr_name: &str) -> Result<&'s Vec<ParamsList>, syn::Error> {
        let attr = self.attrs.get(attr_name);

        if attr.is_none() {
            return Err(syn::Error::new_spanned(
                self.root,
                format!("Attribute {} not found", attr_name),
            ));
        }

        Ok(attr.unwrap())
    }

    pub fn try_get_attrs(&'s self, attr_name: &str) -> Option<&'s Vec<ParamsList>> {
        self.attrs.get(attr_name)
    }

    pub fn get_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<&'s ParamValue, syn::Error> {
        let attr = self.get_attr(attr_name)?;
        attr.get_named_param(param_name)
    }

    pub fn get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<&'s ParamValue, syn::Error> {
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

    pub fn remove(&'s mut self, name: &str) -> Option<Vec<ParamsList>> {
        self.attrs.remove(name)
    }

    pub fn get_attr_names(&'s self) -> std::collections::hash_map::Keys<String, Vec<ParamsList>> {
        self.attrs.keys()
    }
}
