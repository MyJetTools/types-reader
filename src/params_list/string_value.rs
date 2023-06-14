use std::{ops::Deref, str::FromStr};

use proc_macro2::Literal;
#[derive(Debug)]
pub struct StringValue {
    literal: Literal,
    value: String,
}

impl StringValue {
    pub fn new(literal: Literal, value: String) -> Self {
        Self { literal, value }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    pub fn into_string(self) -> String {
        self.value
    }

    pub fn as_literal(&self) -> &Literal {
        &self.literal
    }

    pub fn to_rust_code(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        match proc_macro2::TokenStream::from_str(self.value.as_str()) {
            Ok(token_stream) => Ok(token_stream),
            Err(_) => Err(syn::Error::new_spanned(
                self.as_literal(),
                "Invalid rust code",
            )),
        }
    }
    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(self.as_literal(), message)
    }
}

impl<'s> Into<&'s str> for &'s StringValue {
    fn into(self) -> &'s str {
        self.as_str()
    }
}

impl Into<String> for StringValue {
    fn into(self) -> String {
        self.into_string()
    }
}

impl Deref for StringValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
