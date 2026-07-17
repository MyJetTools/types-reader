use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{OptionalObjectValue, TokensObject};

pub trait MacrosAttribute {
    const NAME: &'static str;
}

pub struct Attributes<'s> {
    attrs: HashMap<String, Vec<TokensObject>>,
    root: &'s dyn ToTokens,
}

impl<'s> Attributes<'s> {
    pub fn new(root: &'s dyn ToTokens, src: &'s [syn::Attribute]) -> Result<Self, syn::Error> {
        let mut attrs = HashMap::new();

        for attr in src {
            let (attr_name, token_stream) = extract_attr_name_and_content(attr)?;

            let attr_name_as_str = attr_name.to_string();

            let param_list = if let Some(token_stream) = token_stream {
                TokensObject::new(token_stream.into())?
            } else {
                TokensObject::create_empty(attr_name)
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

    pub fn check_for_unknown_params(
        &self,
        check: impl Fn(&str, &TokensObject) -> Result<(), syn::Error>,
    ) -> Result<(), syn::Error> {
        for (attr_name, params_list) in &self.attrs {
            for param_list in params_list {
                if let Err(err) = check(attr_name, param_list) {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    pub fn get_attr(&'s self, attr_name: &str) -> Result<&'s TokensObject, syn::Error> {
        let attr = self.attrs.get(attr_name);

        if attr.is_none() {
            return Err(syn::Error::new_spanned(
                self.root,
                format!("Attribute {} not found", attr_name),
            ));
        }

        Ok(attr.unwrap().get(0).unwrap())
    }

    pub fn try_get_attr(&'s self, attr_name: &str) -> Option<&'s TokensObject> {
        let attr = self.attrs.get(attr_name)?;

        Some(attr.get(0).unwrap())
    }

    pub fn get_attrs(&'s self, attr_name: &str) -> Result<&'s Vec<TokensObject>, syn::Error> {
        let attr = self.attrs.get(attr_name);

        if attr.is_none() {
            return Err(syn::Error::new_spanned(
                self.root,
                format!("Attribute {} not found", attr_name),
            ));
        }

        Ok(attr.unwrap())
    }

    pub fn try_get_attrs(&'s self, attr_name: &str) -> Option<&'s Vec<TokensObject>> {
        self.attrs.get(attr_name)
    }

    pub fn get_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<&'s TokensObject, syn::Error> {
        let attr = self.get_attr(attr_name)?;
        attr.get_named_param(param_name)
    }

    pub fn get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<&'s OptionalObjectValue, syn::Error> {
        let attr = self.get_attr(attr_name)?;
        attr.get_value_from_single_or_named(param_name)
    }

    pub fn try_get_single_or_named_param(
        &'s self,
        attr_name: &str,
        param_name: &str,
    ) -> Result<Option<&'s OptionalObjectValue>, syn::Error> {
        match self.try_get_attr(attr_name) {
            Some(attr) => attr.try_get_value_from_single_or_named(param_name),
            None => Ok(None),
        }
    }

    pub fn try_get_single_or_named_params<'d>(
        &'s self,
        attr_name: &str,
        param_names: impl Iterator<Item = &'d str>,
    ) -> Result<Option<&'s OptionalObjectValue>, syn::Error> {
        let attr = self.try_get_attr(attr_name);

        if attr.is_none() {
            return Ok(None);
        }

        let attr = attr.unwrap();

        for param_name in param_names {
            if let Some(value) = attr.try_get_value_from_single_or_named(param_name)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
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

    pub fn remove(&'s mut self, name: &str) -> Option<Vec<TokensObject>> {
        self.attrs.remove(name)
    }

    pub fn get_attr_names(&'s self) -> std::collections::hash_map::Keys<String, Vec<TokensObject>> {
        self.attrs.keys()
    }
}

/// Supported attribute forms are:
/// * `#[name]` - no content;
/// * `#[name(...)]` - content is the token stream inside the brackets;
/// * `#[name = value]` - content is the token stream after the '='. Doc comments are
///   expanded by the compiler into this form: `/// Comment` -> `#[doc = " Comment"]`.
fn extract_attr_name_and_content(
    attr: &syn::Attribute,
) -> Result<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>), syn::Error> {
    let token: proc_macro2::TokenStream = attr.to_token_stream();

    let token = get_inside_attr(attr, token)?;

    let mut tokens = token.into_iter();

    let ident = match tokens.next() {
        Some(ident) => ident,
        None => return Err(syn::Error::new_spanned(attr, "Attribute name is missing")),
    };

    let content_token = tokens.next();

    if content_token.is_none() {
        return Ok((ident.into_token_stream(), None));
    }

    match content_token.unwrap() {
        proc_macro2::TokenTree::Group(value) => Ok((ident.into_token_stream(), Some(value.stream()))),
        proc_macro2::TokenTree::Punct(value) if value.as_char() == '=' => {
            let value_tokens: proc_macro2::TokenStream = tokens.collect();

            if value_tokens.is_empty() {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!("Attribute '{}' has no value after '='", ident),
                ));
            }

            Ok((ident.into_token_stream(), Some(value_tokens)))
        }
        content_token => Err(syn::Error::new_spanned(
            attr,
            format!(
                "Attribute '{}' has unsupported content: '{}'. Supported forms are: #[{0}], #[{0}(...)] and #[{0} = value]",
                ident, content_token
            ),
        )),
    }
}

fn get_inside_attr(attr: &syn::Attribute, token: TokenStream) -> Result<TokenStream, syn::Error> {
    let mut tokens = token.into_iter();

    let sharp = match tokens.next() {
        Some(sharp) => sharp,
        None => return Err(syn::Error::new_spanned(attr, "Attribute is empty")),
    };

    if sharp.to_string() != "#" {
        return Err(syn::Error::new_spanned(
            attr,
            format!("Expected '#' but got '{}'", sharp),
        ));
    }

    let braces_token = match tokens.next() {
        Some(braces_token) => braces_token,
        None => {
            return Err(syn::Error::new_spanned(
                attr,
                "Attribute has no content after '#'",
            ))
        }
    };

    match braces_token {
        proc_macro2::TokenTree::Group(value) => Ok(value.stream()),
        braces_token => Err(syn::Error::new_spanned(
            attr,
            format!("Expected attribute content but got '{}'", braces_token),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::Attributes;

    fn parse(src: &str) -> syn::DeriveInput {
        syn::parse_str(src).unwrap()
    }

    #[test]
    fn test_doc_comment_on_the_field_is_readable() {
        let ast = parse(
            r#"
            pub struct C {
                /// A doc comment on a field.
                pub card_number: u32,
            }
        "#,
        );

        let fields = match &ast.data {
            syn::Data::Struct(data) => &data.fields,
            _ => panic!("Struct is expected"),
        };

        let field = fields.iter().next().unwrap();

        let attrs = Attributes::new(field, &field.attrs).unwrap();

        let value = attrs
            .get_attr("doc")
            .unwrap()
            .unwrap_as_value()
            .unwrap()
            .as_string()
            .unwrap()
            .as_str()
            .to_string();

        assert_eq!(value.trim(), "A doc comment on a field.");
    }

    #[test]
    fn test_name_equals_value_attribute_on_container() {
        let ast = parse(r#"#[deprecated = "use C instead"] pub struct B { pub id: u32 }"#);

        let attrs = Attributes::new(&ast, &ast.attrs).unwrap();

        assert_eq!(
            attrs
                .get_attr("deprecated")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "use C instead"
        );
    }

    #[test]
    fn test_container_attribute_with_list_form_param() {
        let ast = parse(
            r#"
            #[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
            pub struct A { pub id: u32 }
        "#,
        );

        let attrs = Attributes::new(&ast, &ast.attrs).unwrap();

        let rename_all = attrs
            .get_attr("serde")
            .unwrap()
            .get_named_param("rename_all")
            .unwrap();

        assert_eq!(
            rename_all
                .get_named_param("serialize")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "camelCase"
        );

        assert_eq!(
            rename_all
                .get_named_param("deserialize")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "kebab-case"
        );
    }

    #[test]
    fn test_flag_and_group_attribute_forms_still_work() {
        let ast = parse(
            r#"
            #[my_flag]
            #[my_attr(id: 5, name: "test")]
            pub struct A { pub id: u32 }
        "#,
        );

        let attrs = Attributes::new(&ast, &ast.attrs).unwrap();

        assert!(attrs.has_attr("my_flag"));
        assert!(attrs.get_attr("my_flag").unwrap().has_no_value());

        let my_attr = attrs.get_attr("my_attr").unwrap();

        assert_eq!(
            my_attr
                .get_named_param("id")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_number()
                .unwrap()
                .as_i32(),
            5
        );

        assert_eq!(
            my_attr
                .get_named_param("name")
                .unwrap()
                .unwrap_as_value()
                .unwrap()
                .as_string()
                .unwrap()
                .as_str(),
            "test"
        );
    }
}
