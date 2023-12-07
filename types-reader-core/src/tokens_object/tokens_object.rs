use crate::{NextToken, ObjectValue, TokensReader, ValueAsIdent};

use proc_macro2::TokenStream;

use std::collections::HashMap;
#[derive(Debug)]
pub enum TokensObject {
    None(TokenStream),
    ValueOnly(ObjectValue),
    Value {
        param_name: syn::Ident,
        value: ObjectValue,
    },
    Object {
        token_stream: TokenStream,
        items: HashMap<String, TokensObject>,
    },
    Vec {
        token_stream: TokenStream,
        items: Vec<TokensObject>,
    },
}

const SPACE_SYMBOLS: [char; 2] = [';', ','];

impl TokensObject {
    pub fn new(mut token_reader: TokensReader) -> Result<Self, syn::Error> {
        let next_token = token_reader.try_read_next_token()?;

        if next_token.is_none() {
            return Ok(Self::None(token_reader.into_token_stream()));
        }

        let next_token = next_token.unwrap();

        let mut ident_token = match next_token.try_unwrap_as_value() {
            Ok(token_value) => return Ok(Self::ValueOnly(token_value.try_into()?)),
            Err(next_token) => next_token,
        };

        let mut items = HashMap::new();

        while let Ok(param_name) = ident_token.unwrap_into_ident(None) {
            let token_equal = token_reader.try_read_next_token()?;

            if token_equal.is_none() {
                let id = param_name.to_string();
                items.insert(id, Self::None(quote::quote!(#param_name)));
                break;
            }

            let token_equal = token_equal.unwrap();

            if token_equal.if_spacing(Some(&SPACE_SYMBOLS)) {
                let id = param_name.to_string();
                items.insert(id, Self::None(quote::quote!(#param_name)));
            } else if token_equal.if_spacing(Some(&[':', '='])) {
                let token_value = token_reader.read_next_token()?;
                let id = param_name.to_string();
                items.insert(id, Self::read_value(param_name, token_value)?);
            }

            let next_token = token_reader.try_read_next_token()?;
            if next_token.is_none() {
                break;
            }

            ident_token = next_token.unwrap();

            if ident_token.if_spacing(Some(&SPACE_SYMBOLS)) {
                let next_token = token_reader.try_read_next_token()?;
                if next_token.is_none() {
                    break;
                }
                ident_token = next_token.unwrap();
            }
        }

        Ok(Self::Object {
            token_stream: token_reader.into_token_stream(),
            items,
        })
    }

    pub fn create_empty(token_stream: TokenStream) -> Self {
        Self::None(token_stream)
    }

    pub fn has_no_value(&self) -> bool {
        match self {
            TokensObject::None(_) => true,
            _ => false,
        }
    }
    pub fn get_single_param_as_ident(&self) -> Result<&ValueAsIdent, syn::Error> {
        match self {
            Self::None(token_stream) => Err(syn::Error::new_spanned(
                token_stream.clone(),
                "Attribute has no params",
            )),

            Self::ValueOnly(value) => {
                return Err(
                    value.throw_error("Can not get ident. Attribute has parameter as value")
                );
            }

            Self::Value { value, .. } => {
                return Err(
                    value.throw_error("Can not get ident. Attribute has parameter as value")
                );
            }
            Self::Vec { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Can not get ident. Attribute parameters is array",
                ));
            }
            Self::Object { token_stream, .. } => {
                return Err(syn::Error::new_spanned(
                    token_stream.clone(),
                    "Can not get ident. Attribute parameters is object",
                ));
            }
        }
    }

    pub fn check_for_unknown_params(
        &self,
        used_parameters: &[&'static str],
    ) -> Result<(), syn::Error> {
        match self {
            Self::None(_) => return Ok(()),
            Self::ValueOnly(_) => {
                return Ok(());
            }
            Self::Value { .. } => {
                return Ok(());
            }
            Self::Vec { .. } => {
                return Ok(());
            }
            Self::Object {
                items,
                token_stream,
            } => {
                for (param_name, ..) in items {
                    if !used_parameters.iter().any(|itm| *itm == param_name) {
                        return Err(syn::Error::new_spanned(
                            token_stream,
                            format!(
                                "Unknown parameter {}. Parameters are supported: {:?}",
                                param_name, used_parameters
                            ),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn throw_error_at_value_token(&self, message: &str) -> syn::Error {
        match self {
            TokensObject::None(token) => syn::Error::new_spanned(token, message),
            TokensObject::ValueOnly(value) => value.throw_error(message),
            TokensObject::Value { value, .. } => value.throw_error(message),
            TokensObject::Object { token_stream, .. } => {
                syn::Error::new_spanned(token_stream, message)
            }
            TokensObject::Vec { token_stream, .. } => {
                syn::Error::new_spanned(token_stream, message)
            }
        }
    }

    pub fn throw_error_at_param_token(&self, message: &str) -> syn::Error {
        match self {
            TokensObject::None(token) => syn::Error::new_spanned(token, message),
            TokensObject::ValueOnly(value) => value.throw_error(message),
            TokensObject::Value { param_name, .. } => syn::Error::new_spanned(param_name, message),
            TokensObject::Object { token_stream, .. } => {
                syn::Error::new_spanned(token_stream, message)
            }
            TokensObject::Vec { token_stream, .. } => {
                syn::Error::new_spanned(token_stream, message)
            }
        }
    }

    pub fn get_vec(&self) -> Result<&Vec<Self>, syn::Error> {
        match self.try_get_vec() {
            Some(value) => Ok(value),
            None => Err(self.throw_error_at_param_token("Value should be an object list")),
        }
    }

    pub fn try_get_vec(&self) -> Option<&Vec<Self>> {
        match self {
            Self::Vec { items, .. } => Some(items),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Self::Object { .. } => true,
            _ => false,
        }
    }

    pub fn try_get_single_param(&self) -> Option<&ObjectValue> {
        match self {
            Self::ValueOnly(value) => Some(value),
            Self::Value { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn get_named_param(&self, param_name: &str) -> Result<&TokensObject, syn::Error> {
        match self {
            Self::Object { items, .. } => match items.get(param_name) {
                Some(value) => return Ok(value),
                None => {}
            },
            _ => {}
        }

        Err(self.throw_error_at_param_token(format!("Field '{}' is required", param_name).as_str()))
    }

    pub fn try_get_named_param(&self, param_name: &str) -> Option<&TokensObject> {
        match self {
            Self::Object { items, .. } => items.get(param_name),
            _ => None,
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        match self {
            Self::Object { items, .. } => items.contains_key(param_name),
            _ => false,
        }
    }

    pub fn get_from_single_or_named(&self, param_name: &str) -> Result<&ObjectValue, syn::Error> {
        if let Some(result) = self.try_get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)?.get_value()
    }

    pub fn get_value(&self) -> Result<&ObjectValue, syn::Error> {
        match self {
            TokensObject::ValueOnly(value) => Ok(value),
            TokensObject::Value { value, .. } => Ok(value),
            TokensObject::None(_) => {
                Err(self.throw_error_at_param_token("Can not get empty value"))
            }

            TokensObject::Object { .. } => {
                Err(self.throw_error_at_param_token("Can not get value. Value is object"))
            }
            TokensObject::Vec { .. } => {
                Err(self.throw_error_at_param_token("Can not get value. Value is array"))
            }
        }
    }

    pub fn try_get_value(&self) -> Option<&ObjectValue> {
        match self {
            TokensObject::Value { value, .. } => Some(value),
            TokensObject::ValueOnly(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_value_from_single_or_named(
        &self,
        param_name: &str,
    ) -> Result<&ObjectValue, syn::Error> {
        if let Some(result) = self.try_get_single_param() {
            return Ok(result);
        }

        if let Some(result) = self.try_get_named_param(param_name) {
            return result.get_value();
        }

        Err(self.throw_error_at_param_token(format!("Field '{}' is required", param_name).as_str()))
    }

    pub fn try_get_value_from_single_or_named(&self, param_name: &str) -> Option<&ObjectValue> {
        if let Some(result) = self.try_get_single_param() {
            return Some(result);
        }

        self.try_get_named_param(param_name)?.try_get_value()
    }

    pub fn len(&self) -> usize {
        match self {
            Self::None(_) => 0,
            Self::Value { .. } => 1,
            Self::ValueOnly(_) => 1,
            Self::Vec { items, .. } => items.len(),
            Self::Object { items, .. } => items.len(),
        }
    }

    pub fn is_vec(&self) -> bool {
        match self {
            Self::Vec { .. } => true,
            _ => false,
        }
    }

    pub fn unwrap_as_object(&self) -> &HashMap<String, TokensObject> {
        match self {
            Self::Object { items, .. } => items,
            _ => panic!("Can not unwrap as object"),
        }
    }

    fn read_value(param_name: syn::Ident, token_value: NextToken) -> Result<Self, syn::Error> {
        let next_token = match token_value.try_unwrap_as_value() {
            Ok(token_value) => {
                return Ok(Self::Value {
                    param_name,
                    value: token_value.try_into()?,
                })
            }
            Err(next_token) => next_token,
        };

        let next_token = match next_token.try_unwrap_into_ident() {
            Ok(ident) => {
                return Ok(Self::Value {
                    param_name,
                    value: ident.try_into()?,
                })
            }
            Err(next_token) => next_token,
        };

        let next_token = match next_token.try_unwrap_into_group(None) {
            Ok((group_tokens, delimiter)) => match delimiter {
                proc_macro2::Delimiter::Bracket => {
                    let (items, token_stream) = Self::parse_as_array(param_name, group_tokens)?;
                    return Ok(Self::Vec {
                        token_stream,
                        items,
                    });
                }
                proc_macro2::Delimiter::Brace => {
                    return Ok(Self::new(group_tokens)?);
                }
                _ => panic!(
                    "Value can not be parsed from group {:?} of tokens",
                    delimiter
                ),
            },
            Err(next_token) => next_token,
        };

        Err(next_token.throw_error("Invalid value to read"))
    }

    pub fn parse_as_array(
        param_name: syn::Ident,
        mut token_reader: TokensReader,
    ) -> Result<(Vec<TokensObject>, TokenStream), syn::Error> {
        let mut result: Vec<TokensObject> = Vec::new();

        while let Some(mut token) = token_reader.try_read_next_token()? {
            if token.if_spacing(Some(&SPACE_SYMBOLS)) {
                let next_token = token_reader.try_read_next_token()?;
                if next_token.is_none() {
                    break;
                }

                token = next_token.unwrap();
            }

            let param_value = Self::read_value(param_name.clone(), token)?;
            result.push(param_value);
        }

        Ok((result, token_reader.into_token_stream()))
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}

impl TryInto<TokensObject> for proc_macro2::TokenStream {
    type Error = syn::Error;

    fn try_into(self) -> Result<TokensObject, Self::Error> {
        TokensObject::new(self.into())
    }
}

impl AsRef<TokensObject> for TokensObject {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::TokensObject;

    #[test]
    fn test_value_in_named_param_reading_by_single_or_by_name_but_topic() {
        let src = r#"topic_id = "bid-ask""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list
            .try_get_named_param("topic_id")
            .unwrap()
            .get_value()
            .unwrap()
            .as_string()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);

        let value = params_list
            .try_get_value_from_single_or_named("topic_id")
            .unwrap()
            .as_string()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);
    }

    #[test]
    fn test_value_in_single_param_reading_by_single_or_by_name_but_topic() {
        let src = r#""bid-ask""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list
            .try_get_value_from_single_or_named("topic_id")
            .unwrap()
            .as_string()
            .unwrap()
            .as_str();

        assert_eq!("bid-ask", value);
    }

    #[test]
    fn test_empty_brackets() {
        let src = r#"authorized: []"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.try_get_named_param("authorized").unwrap();

        assert!(value.is_vec());
    }

    #[test]
    fn test_with_empty_value() {
        let src = r#"id: 5; name:"5";  description:"Persist during 5 sec"; default"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.try_get_named_param("id").unwrap();
        assert_eq!(value.get_value().unwrap().as_number().unwrap().as_i32(), 5);

        let value = params_list.try_get_named_param("name").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "5"
        );

        let value = params_list.try_get_named_param("description").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Persist during 5 sec"
        );

        assert!(params_list.has_param("default"));
    }

    #[test]
    fn test_with_negative_values() {
        let src = r#"id = -1; description = "Table already exists""#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.try_get_named_param("id").unwrap();
        assert_eq!(value.get_value().unwrap().as_number().unwrap().as_i32(), -1);

        let value = params_list.try_get_named_param("description").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Table already exists"
        );
    }
    #[test]
    fn test_with_single_negative_value() {
        let src = r#"-256"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let tokens_object = TokensObject::new(token_stream.into()).unwrap();

        let value = tokens_object.get_value().unwrap();

        assert_eq!(value.as_number().unwrap().as_i32(), -256);
    }

    #[test]
    fn test_with_single_positive_value() {
        let src = r#"256"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let tokens_object = TokensObject::new(token_stream.into()).unwrap();

        let value = tokens_object.get_value().unwrap();

        assert_eq!(value.as_number().unwrap().as_i32(), 256);
    }

    #[test]
    fn test_with_single_positive_double_value() {
        let src = r#"256.34"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_value().unwrap();

        assert_eq!(value.as_double().unwrap().as_f64(), 256.34);
    }

    #[test]
    fn test_with_single_negative_double_value() {
        let src = r#"-256.34"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_value().unwrap();

        assert_eq!(value.as_double().unwrap().as_f64(), -256.34);
    }

    #[test]
    fn test_with_boolean_value_as_true() {
        let src = r#"description = "Persist table"; default: true"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_named_param("description").unwrap();

        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Persist table"
        );

        let value = params_list.get_named_param("default").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_bool().unwrap().get_value(),
            true
        );
    }

    #[test]
    fn test_with_boolean_value_as_false() {
        let src = r#"description = "Persist table"; default: false"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_named_param("description").unwrap();

        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Persist table"
        );

        let value = params_list.get_named_param("default").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_bool().unwrap().get_value(),
            false
        );
    }

    #[test]
    fn test_with_object_inside() {
        let src = r#"string_param: "Persist table"; object_param: {first_object_param:1, second_object_param:true}"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_named_param("string_param").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Persist table"
        );

        let object = params_list.get_named_param("object_param").unwrap();

        let value = object.get_named_param("first_object_param").unwrap();
        assert_eq!(value.get_value().unwrap().as_number().unwrap().as_i32(), 1);

        let value = object.get_named_param("second_object_param").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_bool().unwrap().get_value(),
            true
        );
    }

    #[test]
    fn test_with_array_of_objects_inside() {
        let src = r#"first_param: "Value"; array_of_object: [{first_object_param:1, second_object_param:true},{first_object_param:2, second_object_param:false}]"#;

        let token_stream = proc_macro2::TokenStream::from_str(src).unwrap();

        let params_list = TokensObject::new(token_stream.into()).unwrap();

        let value = params_list.get_named_param("first_param").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_string().unwrap().as_str(),
            "Value"
        );

        let array_value = params_list.get_named_param("array_of_object").unwrap();
        let objects = array_value.get_vec().unwrap();

        assert_eq!(2, objects.len());

        let first_object = objects.get(0).unwrap();

        let value = first_object.get_named_param("first_object_param").unwrap();
        assert_eq!(value.get_value().unwrap().as_number().unwrap().as_i32(), 1);

        let value = first_object.get_named_param("second_object_param").unwrap();
        assert_eq!(
            value.get_value().unwrap().as_bool().unwrap().get_value(),
            true
        );

        let second_object = objects.get(1).unwrap();

        let value = second_object.get_named_param("first_object_param").unwrap();
        assert_eq!(value.get_value().unwrap().as_number().unwrap().as_i32(), 2);

        let value = second_object
            .get_named_param("second_object_param")
            .unwrap();
        assert_eq!(
            value.get_value().unwrap().as_bool().unwrap().get_value(),
            false
        );
    }
}
