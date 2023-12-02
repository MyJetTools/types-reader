use proc_macro2::Literal;

#[derive(Debug)]
pub struct TokenValue {
    inner: Literal,
    pub negative_value: bool,
}

impl TokenValue {
    pub fn new(value: Literal, negative_value: bool) -> Self {
        Self {
            inner: value,
            negative_value,
        }
    }

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn as_literal(&self) -> &Literal {
        &self.inner
    }
}
