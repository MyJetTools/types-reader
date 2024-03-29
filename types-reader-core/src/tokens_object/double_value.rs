use proc_macro2::Literal;

use crate::TokenValue;
#[derive(Debug)]
pub struct DoubleValue {
    literal: TokenValue,
    value: f64,
    str_value: String,
}

impl DoubleValue {
    pub fn new(literal: TokenValue, value: f64, str_value: String) -> Self {
        Self {
            literal,
            value,
            str_value,
        }
    }

    pub fn as_str(&self) -> &str {
        self.str_value.as_str()
    }

    pub fn as_literal(&self) -> &Literal {
        self.literal.as_literal()
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn as_f32(&self) -> f32 {
        self.value as f32
    }

    pub fn as_u16(&self) -> u16 {
        self.value as u16
    }

    pub fn as_i8(&self) -> i8 {
        self.value as i8
    }

    pub fn as_u8(&self) -> u8 {
        self.value as u8
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(self.as_literal(), message)
    }
}

impl<'s> Into<f32> for &'s DoubleValue {
    fn into(self) -> f32 {
        self.as_f32()
    }
}

impl<'s> Into<f64> for &'s DoubleValue {
    fn into(self) -> f64 {
        self.as_f64()
    }
}
