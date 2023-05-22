use crate::{ObjectsIteratorShared, ParamContent};

pub struct ObjectsIterator<'s> {
    pub src: &'s [u8],
    pub pos: usize,
    pub opener: u8,
}

impl<'s> ObjectsIterator<'s> {
    pub fn new(src: &'s str) -> Self {
        let src = src.as_bytes();
        let opener = super::utils::get_opener(src);
        Self {
            src,
            pos: 1,
            opener,
        }
    }

    pub fn get_next(&mut self) -> Option<ParamContent<'s>> {
        let object_start_pos = self.find_object_start()?;

        let object_end_pos = self.find_value_end_as_array_or_object(b'{')?;

        let result = &self.src[object_start_pos..object_end_pos];

        Some(ParamContent::Object(std::str::from_utf8(result).unwrap()))
    }

    fn find_object_start(&mut self) -> Option<usize> {
        while let Some(b) = self.get_current_byte() {
            if b == b'{' {
                return Some(self.pos);
            }

            self.inc_pos();
        }

        None
    }
}

impl<'s> Iterator for ObjectsIterator<'s> {
    type Item = ParamContent<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next()
    }
}

impl<'s> ObjectsIteratorShared for ObjectsIterator<'s> {
    fn get_current_byte(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn get_payload_len(&self) -> usize {
        self.src.len()
    }

    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_opener(&self) -> u8 {
        self.opener
    }

    fn inc_pos(&mut self) {
        self.pos += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::ObjectsIterator;

    #[test]
    fn test() {
        let src = r#"[{a:1,b:2,c:'3'}, {a:5,b:'6',c:7}]"#;

        let mut objects = ObjectsIterator::new(src);

        let result = objects.get_next().unwrap();

        assert!(result.is_object());
        assert_eq!(result.as_raw_str(), "{a:1,b:2,c:'3'}");

        let result = objects.get_next().unwrap();

        assert!(result.is_object());
        assert_eq!(result.as_raw_str(), "{a:5,b:'6',c:7}");

        let result = objects.get_next();

        assert!(result.is_none());
    }
}
