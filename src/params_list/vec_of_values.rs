use proc_macro2::{Literal, TokenStream};

use crate::ParamValue;
#[derive(Debug)]
pub struct VecOfValues {
    token_stream: TokenStream,
    value: Vec<ParamValue>,
}

impl VecOfValues {
    pub fn new(token_stream: TokenStream) -> Self {
        Self {
            token_stream,
            value: Vec::new(),
        }
    }

    pub fn add_value(&mut self, value: Literal, is_negative: bool) -> Result<(), syn::Error> {
        let value = ParamValue::from_literal(value, is_negative)?;
        self.value.push(value);
        Ok(())
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(&self.token_stream, message)
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &ParamValue> {
        self.value.iter()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}