use proc_macro2::Literal;

pub struct BoolValue {
    literal: Literal,
    value: bool,
    str_value: String,
}

impl BoolValue {
    pub fn new(literal: Literal, value: bool, str_value: String) -> Self {
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
        &self.literal
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    pub fn throw_error(&self, message: &str) -> syn::Error {
        syn::Error::new_spanned(self.as_literal(), message)
    }
}
