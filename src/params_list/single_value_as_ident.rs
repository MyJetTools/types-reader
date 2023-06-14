use proc_macro2::Ident;
#[derive(Debug)]
pub struct SingleValueAsIdent {
    ident: Ident,
    value: String,
}

impl SingleValueAsIdent {
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
