use crate::{get_list_of_elements, SingleValueAsIdent};

use super::ParamValue;
use proc_macro2::{Ident, TokenStream, TokenTree};
use std::collections::{HashMap, VecDeque};
#[derive(Debug)]
pub enum ParamsList {
    None(TokenStream),
    Single {
        token_stream: TokenStream,
        value: ParamValue,
    },
    Multiple {
        token_stream: TokenStream,
        items: HashMap<String, ParamValue>,
    },
}

impl ParamsList {
    pub fn new(token_stream: TokenStream) -> Result<Self, syn::Error> {
        let mut tokens: VecDeque<TokenTree> = token_stream.clone().into_iter().collect();

        if tokens.len() == 0 {
            return Ok(Self::None(token_stream));
        }

        if tokens.len() == 1 {
            return Self::from_single_token(token_stream, tokens, false);
        }

        if tokens.len() == 2 {
            let token = tokens.pop_front().unwrap();

            if token.to_string() != "-" {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Invalid value",
                ));
            }
            return Self::from_single_token(token_stream, tokens, true);
        }

        let mut items = HashMap::new();

        while let Some(ident) = get_ident(&mut tokens)? {
            let key: String = ident.to_string();

            let value = read_value(ident, &mut tokens)?;

            items.insert(key, value);
        }

        Ok(Self::Multiple {
            token_stream,
            items,
        })
    }

    pub fn create_empty(token_stream: TokenStream) -> Self {
        Self::None(token_stream)
    }
    pub fn get_single_param(&self) -> Result<&ParamValue, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),
            Self::Single { value, .. } => Ok(value),
            Self::Multiple { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Attribute has multiple params",
                ));
            }
        }
    }

    pub fn check_for_unknown_params(
        &self,
        used_attributes: &[&'static str],
    ) -> Result<(), syn::Error> {
        match self {
            Self::None(_) => return Ok(()),
            Self::Single { .. } => {
                return Ok(());
            }
            Self::Multiple { items, .. } => {
                for (param_name, value) in items {
                    if !used_attributes.iter().any(|itm| *itm == param_name) {
                        return Err(
                            value.throw_error(format!("Unknown parameter {}", param_name).as_str())
                        );
                    }
                }
            }
        }

        Ok(())
    }

    fn from_single_token(
        token_stream: TokenStream,
        mut tokens: VecDeque<TokenTree>,
        is_negative: bool,
    ) -> Result<Self, syn::Error> {
        let token = tokens.pop_front().unwrap();
        match token {
            TokenTree::Group(el) => {
                println!("Group: {}", el.to_string());
                panic!("Single element can not be group")
            }
            TokenTree::Ident(ident) => {
                let value = ident.to_string();
                return Ok(Self::Single {
                    token_stream,
                    value: ParamValue::SingleValueAsIdent(SingleValueAsIdent::new(ident, value)),
                });
            }
            TokenTree::Punct(el) => {
                println!("Punct: {:?}", el.to_string());
                panic!("Single element can not be punct");
            }
            TokenTree::Literal(literal) => {
                return Ok(Self::Single {
                    token_stream,
                    value: ParamValue::from_literal(literal, is_negative)?,
                });
            }
        }
    }

    pub fn try_get_single_param(&self) -> Option<&ParamValue> {
        match self {
            Self::None(_) => None,
            Self::Single { value, .. } => Some(value),
            Self::Multiple { .. } => None,
        }
    }

    pub fn get_named_param(&self, param_name: &str) -> Result<&ParamValue, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),
            Self::Single { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Named fields are required",
                ));
            }
            Self::Multiple {
                token_stream,
                items,
            } => match items.get(param_name) {
                Some(value) => return Ok(value),
                None => {
                    return Err(syn::Error::new_spanned(
                        token_stream.clone(),
                        format!("Field '{}' is required", param_name),
                    ));
                }
            },
        }
    }

    pub fn try_get_named_param(&self, param_name: &str) -> Option<&ParamValue> {
        match self {
            Self::Multiple { items, .. } => items.get(param_name),
            _ => None,
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        match self {
            Self::Multiple { items, .. } => items.contains_key(param_name),
            _ => false,
        }
    }

    pub fn get_from_single_or_named(&self, param_name: &str) -> Result<&ParamValue, syn::Error> {
        if let Some(result) = self.try_get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }

    pub fn try_get_from_single_or_named(&self, param_name: &str) -> Option<&ParamValue> {
        if let Some(result) = self.try_get_single_param() {
            return Some(result);
        }

        self.try_get_named_param(param_name)
    }

    pub fn get_token_stream(&self) -> &TokenStream {
        match self {
            Self::None(token_stream) => token_stream,
            Self::Single { token_stream, .. } => token_stream,
            Self::Multiple { token_stream, .. } => token_stream,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::None(_) => 0,
            Self::Single { .. } => 1,
            Self::Multiple { items, .. } => items.len(),
        }
    }
}

fn get_ident(items: &mut VecDeque<TokenTree>) -> Result<Option<Ident>, syn::Error> {
    while let Some(token_tree) = items.pop_front() {
        match token_tree {
            TokenTree::Ident(value) => return Ok(Some(value)),
            TokenTree::Literal(value) => {
                let str = value.to_string();
                return Err(syn::Error::new_spanned(
                    value,
                    format!("Expected ident but got literal {} ", str),
                ));
            }
            TokenTree::Punct(_) => {}

            TokenTree::Group(value) => {
                let str = value.to_string();
                return Err(syn::Error::new_spanned(
                    value,
                    format!("Expected ident but got group {} ", str),
                ));
            }
        }
    }

    Ok(None)
}

pub enum IntoValueResult {
    Minus,
    Value(ParamValue),
}

fn read_value(ident: Ident, tokens: &mut VecDeque<TokenTree>) -> Result<ParamValue, syn::Error> {
    let mut is_negative = false;
    loop {
        let token_tree = tokens.pop_front();

        if token_tree.is_none() {
            return Ok(ParamValue::None(ident.clone()));
        }

        match token_tree.unwrap() {
            TokenTree::Ident(ident) => {
                return ParamValue::from_ident(ident);
            }
            TokenTree::Group(value) => match value.delimiter() {
                proc_macro2::Delimiter::Parenthesis => {
                    panic!(
                        "Not implemented group with Delimiter = Parenthesis. Value {}",
                        value.to_string()
                    )
                }
                proc_macro2::Delimiter::Brace => {
                    let token_stream = value.stream();
                    let value = ParamsList::new(token_stream.clone())?;
                    let result = ParamValue::Object {
                        token_stream,
                        value: Box::new(value),
                    };
                    return Ok(result);
                }
                proc_macro2::Delimiter::Bracket => {
                    let token_stream = value.stream();
                    let value = get_list_of_elements(token_stream.clone())?;

                    return Ok(value);
                }
                proc_macro2::Delimiter::None => {
                    panic!(
                        "Not implemented group with Delimiter = None. Value {}",
                        value.to_string()
                    )
                }
            },
            TokenTree::Punct(value) => {
                if value.as_char() == '-' {
                    is_negative = true;
                }
            }
            TokenTree::Literal(value) => {
                return Ok(ParamValue::from_literal(value, is_negative)?);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ParamsList;

    #[test]
    fn test_value_in_named_param_reading_by_single_or_by_name_but_topic() {
        let src = r#"topic_id = "bid-ask""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list
            .try_get_named_param("topic_id")
            .unwrap()
            .unwrap_as_string_value()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);

        let value = params_list
            .try_get_from_single_or_named("topic_id")
            .unwrap()
            .unwrap_as_string_value()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);
    }

    #[test]
    fn test_value_in_single_param_reading_by_single_or_by_name_but_topic() {
        let src = r#""bid-ask""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list
            .try_get_from_single_or_named("topic_id")
            .unwrap()
            .unwrap_as_string_value()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);
    }

    #[test]
    fn test_empty_brackets() {
        let src = r#"authorized: []"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.try_get_named_param("authorized").unwrap();

        assert!(value.is_vec_of_values());
    }

    #[test]
    fn test_with_empty_value() {
        let src = r#"id: 5; name:"5";  description:"Persist during 5 sec"; default"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.try_get_named_param("id").unwrap();
        assert_eq!(value.unwrap_as_number_value().unwrap().as_i32(), 5);

        let value = params_list.try_get_named_param("name").unwrap();
        assert_eq!(value.unwrap_as_string_value().unwrap().as_str(), "5");

        let value = params_list.try_get_named_param("description").unwrap();
        assert_eq!(
            value.unwrap_as_string_value().unwrap().as_str(),
            "Persist during 5 sec"
        );

        assert!(params_list.has_param("default"));
    }

    #[test]
    fn test_with_negative_values() {
        let src = r#"id = -1; description = "Table already exists""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.try_get_named_param("id").unwrap();
        assert_eq!(value.unwrap_as_number_value().unwrap().as_i32(), -1);

        let value = params_list.try_get_named_param("description").unwrap();
        assert_eq!(
            value.unwrap_as_string_value().unwrap().as_str(),
            "Table already exists"
        );
    }
    #[test]
    fn test_with_single_negative_value() {
        let src = r#"-256"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_single_param().unwrap();

        assert_eq!(value.unwrap_as_number_value().unwrap().as_i32(), -256);
    }

    #[test]
    fn test_with_single_positive_value() {
        let src = r#"256"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_single_param().unwrap();

        assert_eq!(value.unwrap_as_number_value().unwrap().as_i32(), 256);
    }

    #[test]
    fn test_with_single_positive_double_value() {
        let src = r#"256.34"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_single_param().unwrap();

        assert_eq!(value.unwrap_as_double_value().unwrap().as_f64(), 256.34);
    }

    #[test]
    fn test_with_single_negative_double_value() {
        let src = r#"-256.34"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_single_param().unwrap();

        assert_eq!(value.unwrap_as_double_value().unwrap().as_f64(), -256.34);
    }

    #[test]
    fn test_with_boolean_value_as_true() {
        let src = r#"description = "Persist table"; default: true"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_named_param("description").unwrap();

        assert_eq!(
            value.unwrap_as_string_value().unwrap().as_str(),
            "Persist table"
        );

        let value = params_list.get_named_param("default").unwrap();
        assert_eq!(value.unwrap_as_bool_value().unwrap().get_value(), true);
    }

    #[test]
    fn test_with_boolean_value_as_false() {
        let src = r#"description = "Persist table"; default: false"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = ParamsList::new(token_stream).unwrap();

        let value = params_list.get_named_param("description").unwrap();

        assert_eq!(
            value.unwrap_as_string_value().unwrap().as_str(),
            "Persist table"
        );

        let value = params_list.get_named_param("default").unwrap();
        assert_eq!(value.unwrap_as_bool_value().unwrap().get_value(), false);
    }
}
