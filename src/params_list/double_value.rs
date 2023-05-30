use proc_macro2::Literal;

pub struct DoubleValue {
    literal: Literal,
    value: f64,
    str_value: String,
}

impl DoubleValue {
    pub fn new(literal: Literal, value: f64, str_value: String) -> Self {
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
}
