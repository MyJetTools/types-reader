use std::collections::HashMap;

use proc_macro2::TokenStream;
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
            let (attr_name, token_stream) = extract_attr_name_and_content(attr);

            let attr_name_as_str = attr_name.to_string();

            let param_list = if let Some(token_stream) = token_stream {
                ParamsList::new(token_stream)?
            } else {
                ParamsList::create_empty(attr_name)
            };

            if !attrs.contains_key(&attr_name_as_str) {
                attrs.insert(attr_name_as_str.clone(), Vec::new());
            }
            attrs
                .get_mut(attr_name_as_str.as_str())
                .unwrap()
                .push(param_list);
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

    pub fn try_get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Option<&'s ParamValue> {
        let attr = self.try_get_attr(attr_name)?;
        attr.try_get_from_single_or_named(param_name)
    }

    pub fn try_get_single_or_named_params<'d>(
        &'s self,
        attr_name: &str,
        param_names: impl Iterator<Item = &'d str>,
    ) -> Option<&'s ParamValue> {
        let attr = self.try_get_attr(attr_name)?;

        for param_name in param_names {
            if let Some(value) = attr.try_get_from_single_or_named(param_name) {
                return Some(value);
            }
        }

        None
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

fn extract_attr_name_and_content(
    attr: &syn::Attribute,
) -> (proc_macro2::TokenStream, Option<proc_macro2::TokenStream>) {
    let token: proc_macro2::TokenStream = attr.to_token_stream();

    let token = get_inside_attr(token);

    let mut tokens = token.into_iter();

    let ident = tokens.next().unwrap();

    let braces_token = tokens.next();

    if braces_token.is_none() {
        return (ident.into_token_stream(), None);
    }

    match braces_token.unwrap() {
        proc_macro2::TokenTree::Group(value) => {
            let token = value.stream();

            return (ident.into_token_stream(), Some(token));
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

fn get_inside_attr(token: TokenStream) -> TokenStream {
    let mut tokens = token.into_iter();

    let ident = tokens.next().unwrap();

    let name = ident.to_string();

    if name != "#" {
        panic!("Expected '#'");
    }

    let braces_token = tokens.next().unwrap();

    match braces_token {
        proc_macro2::TokenTree::Group(value) => {
            return value.stream();
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
