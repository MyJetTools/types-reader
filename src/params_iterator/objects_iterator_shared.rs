use crate::ValueType;

pub trait ObjectsIteratorShared {
    fn get_current_byte(&self) -> Option<u8>;
    fn get_payload_len(&self) -> usize;
    fn get_pos(&self) -> usize;

    fn get_opener(&self) -> u8;

    fn inc_pos(&mut self);

    fn find_value_start(&mut self) -> Option<ValueType> {
        let opener = self.get_opener();
        while let Some(b) = self.get_current_byte() {
            if super::utils::is_closer(opener, b) {
                return Some(ValueType::Empty);
            }

            if super::utils::is_value_separator(b) {
                return Some(ValueType::Empty);
            }

            if !super::utils::is_param_name_separator(b) && b >= 32 {
                if let Some(result) = super::utils::is_beginning_of_value(b) {
                    return Some(result);
                }
            }
            self.inc_pos();
        }

        None
    }

    fn find_end_of_number(&mut self) -> Option<usize> {
        let opener = self.get_opener();
        while let Some(b) = self.get_current_byte() {
            if b <= 32 || super::utils::is_value_separator(b) || super::utils::is_closer(opener, b)
            {
                return Some(self.get_pos());
            }
            self.inc_pos()
        }

        None
    }

    fn find_end_of_string(&mut self) -> Option<usize> {
        let string_open_quote = self.get_current_byte()?;
        self.inc_pos();
        while let Some(b) = self.get_current_byte() {
            if b == b'\\' {
                self.inc_pos();
                self.inc_pos();
                continue;
            }

            if b == string_open_quote {
                self.inc_pos();
                return Some(self.get_pos());
            }
            self.inc_pos();
        }

        None
    }

    fn find_value_end_as_bool(&mut self) -> Option<usize> {
        let opener = self.get_opener();
        while let Some(b) = self.get_current_byte() {
            if b <= 32 || super::utils::is_value_separator(b) || super::utils::is_closer(opener, b)
            {
                return Some(self.get_pos());
            }
            self.inc_pos();
        }

        None
    }

    fn skip_separator(&mut self) -> Option<usize> {
        let opener = self.get_opener();
        while let Some(b) = self.get_current_byte() {
            if super::utils::is_closer(opener, b) {
                return None;
            }

            if b >= 32 && !super::utils::is_value_separator(b) {
                return Some(self.get_pos());
            }
            self.inc_pos();
        }

        None
    }

    fn find_value_end(&mut self, value_type: &ValueType) -> Option<usize> {
        match value_type {
            ValueType::Number => self.find_end_of_number(),
            ValueType::String => self.find_end_of_string(),
            ValueType::Bool => self.find_value_end_as_bool(),
            ValueType::Array => self.find_value_end_as_array_or_object(b'['),
            ValueType::Object => self.find_value_end_as_array_or_object(b'{'),
            ValueType::Empty => None,
        }
    }

    fn find_value_end_as_array_or_object(&mut self, opener: u8) -> Option<usize> {
        while let Some(b) = self.get_current_byte() {
            if b == b'"' || b == b'\'' {
                self.find_end_of_string();
                continue;
            }

            if super::utils::is_closer(opener, b) {
                self.inc_pos();
                return Some(self.get_pos());
            }
            self.inc_pos();
        }

        None
    }

    fn find_param_name_end(&mut self) -> Option<usize> {
        let opener = self.get_opener();
        while let Some(b) = self.get_current_byte() {
            if b <= 32
                || super::utils::is_param_name_separator(b)
                || super::utils::is_value_separator(b)
                || super::utils::is_closer(opener, b)
            {
                return Some(self.get_pos());
            }

            self.inc_pos();
        }

        None
    }

    fn find_param_name_start(&mut self) -> Option<usize> {
        while let Some(b) = self.get_current_byte() {
            if b > 32 {
                return Some(self.get_pos());
            }

            self.inc_pos();
        }

        None
    }
}
