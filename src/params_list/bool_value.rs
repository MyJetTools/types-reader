use syn::Ident;
#[derive(Debug)]
pub struct BoolValue {
    ident: Ident,
    value: bool,
    str_value: String,
}

impl BoolValue {
    pub fn new(ident: Ident, value: bool, str_value: String) -> Self {
        Self {
            ident,
            value,
            str_value,
        }
    }

    pub fn as_str(&self) -> &str {
        self.str_value.as_str()
    }

    pub fn as_ident(&self) -> &Ident {
        &self.ident
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(self.as_ident(), message)
    }
}

impl<'s> Into<bool> for &'s BoolValue {
    fn into(self) -> bool {
        self.value
    }
}
