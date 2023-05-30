use proc_macro2::Ident;

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
}
