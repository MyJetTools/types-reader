use std::collections::HashMap;

use crate::{ParamContent, ParamsIterator};

pub enum ComplexAttrParams<'s> {
    None,
    Single(ParamContent<'s>),
    Multiple(HashMap<&'s str, ParamContent<'s>>),
}

impl<'s> ComplexAttrParams<'s> {
    pub fn new(src: &'s mut String) -> Self {
        if !src.starts_with('(') {
            src.insert(0, '(');
            src.push(')');
        }
        if src == "()" {
            return Self::None;
        }

        if let Some(single_value) = is_single_value(src) {
            return Self::Single(single_value);
        }
        let params = ParamsIterator::new(src).into_has_map();
        Self::Multiple(params)
    }

    pub fn get_single_param(&'s self) -> Result<ParamContent<'s>, String> {
        match self {
            Self::None => Err("Attribute has no params".to_string()),
            Self::Single(value) => Ok(value.clone()),
            Self::Multiple { .. } => Err("Attribute has multiple params".to_string()),
        }
    }

    pub fn get_named_param(&'s self, param_name: &str) -> Result<ParamContent<'s>, String> {
        match self {
            Self::None => Err("Attribute has no params".to_string()),
            Self::Single(_) => Err(format!("Attribute has single param")),
            Self::Multiple(values) => match values.get(param_name) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Attribute has no param with name '{}'", param_name)),
            },
        }
    }

    pub fn try_get_named_param(&'s self, param_name: &str) -> Option<ParamContent<'s>> {
        match self {
            Self::None => None,
            Self::Single(_) => None,
            Self::Multiple(values) => values.get(param_name).cloned(),
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        if let Self::Multiple(values) = self {
            return values.contains_key(param_name);
        }

        false
    }

    pub fn get_from_single_or_named(
        &'s self,
        param_name: &str,
    ) -> Result<ParamContent<'s>, String> {
        if let Ok(result) = self.get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }

    pub fn has_params_at_all(&self) -> bool {
        match self {
            Self::None => false,
            Self::Single(_) => true,
            Self::Multiple(_) => true,
        }
    }
}

fn is_single_value(src: &str) -> Option<ParamContent<'_>> {
    if src.starts_with("(\"") {
        let as_bytes = src.as_bytes();
        let src = as_bytes[1..as_bytes.len() - 1].as_ref();
        return Some(ParamContent::String(std::str::from_utf8(src).unwrap()));
    }
    None
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_is_single_value() {
        let src = "(\"hello\")";
        let result = super::is_single_value(src).unwrap();
        assert_eq!("\"hello\"", result.as_raw_str());
    }
}
