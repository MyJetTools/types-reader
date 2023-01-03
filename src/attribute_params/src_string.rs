pub struct SrcString {
    value: String,
}

impl SrcString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn get_str(&self) -> &str {
        if self.value.len() == 0 {
            return "";
        }

        if self.value.starts_with('(') {
            &self.value[1..self.value.len() - 1]
        } else {
            &self.value
        }
    }
}
