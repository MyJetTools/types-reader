use proc_macro2::Literal;
use quote::ToTokens;

use crate::TokenValue;
#[derive(Debug)]
pub struct NumberValue {
    literal: TokenValue,
    value: i64,
    str_value: String,
}

impl NumberValue {
    pub fn new(literal: TokenValue, value: i64, str_value: String) -> Self {
        Self {
            literal,
            value,
            str_value,
        }
    }

    pub fn as_literal(&self) -> &Literal {
        self.literal.as_literal()
    }

    pub fn as_str(&self) -> &str {
        self.str_value.as_str()
    }

    pub fn as_i64(&self) -> i64 {
        self.value
    }

    pub fn as_u64(&self) -> u64 {
        self.value as u64
    }

    pub fn as_i32(&self) -> i32 {
        self.value as i32
    }

    pub fn as_u32(&self) -> u32 {
        self.value as u32
    }

    pub fn as_i16(&self) -> i16 {
        self.value as i16
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

    pub fn as_usize(&self) -> usize {
        self.value as usize
    }

    pub fn as_isize(&self) -> isize {
        self.value as isize
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(self.as_literal(), message)
    }
}

impl<'s> Into<u8> for &'s NumberValue {
    fn into(self) -> u8 {
        self.as_u8()
    }
}

impl<'s> Into<i8> for &'s NumberValue {
    fn into(self) -> i8 {
        self.as_i8()
    }
}

impl<'s> Into<u16> for &'s NumberValue {
    fn into(self) -> u16 {
        self.as_u16()
    }
}

impl<'s> Into<i16> for &'s NumberValue {
    fn into(self) -> i16 {
        self.as_i16()
    }
}

impl<'s> Into<u32> for &'s NumberValue {
    fn into(self) -> u32 {
        self.as_u32()
    }
}

impl<'s> Into<i32> for &'s NumberValue {
    fn into(self) -> i32 {
        self.as_i32()
    }
}

impl<'s> Into<u64> for &'s NumberValue {
    fn into(self) -> u64 {
        self.as_u64()
    }
}

impl<'s> Into<i64> for &'s NumberValue {
    fn into(self) -> i64 {
        self.as_i64()
    }
}

impl<'s> Into<usize> for &'s NumberValue {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl<'s> Into<isize> for &'s NumberValue {
    fn into(self) -> isize {
        self.as_isize()
    }
}

impl ToTokens for NumberValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens)
    }
}
