use std::ops::Deref;

use proc_macro2::Ident;
#[derive(Debug)]
pub struct ValueAsIdent {
    ident: Ident,
    value: String,
}

impl ValueAsIdent {
    pub fn new(ident: Ident, value: String) -> Self {
        Self { ident, value }
    }

    pub fn get_ident(&self) -> &Ident {
        &self.ident
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(&self.ident, message)
    }
}

impl<'s> Into<&'s str> for &'s ValueAsIdent {
    fn into(self) -> &'s str {
        self.as_str()
    }
}

impl<'s> Into<String> for &'s ValueAsIdent {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl Deref for ValueAsIdent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
