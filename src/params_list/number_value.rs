use proc_macro2::Literal;

pub struct NumberValue {
    literal: Literal,
    value: i64,
    str_value: String,
}

impl NumberValue {
    pub fn new(literal: Literal, value: i64, str_value: String) -> Self {
        Self {
            literal,
            value,
            str_value,
        }
    }

    pub fn as_literal(&self) -> &Literal {
        &self.literal
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
