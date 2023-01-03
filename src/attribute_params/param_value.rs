use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ParamValue<'s> {
    pub value: &'s [u8],
}

impl<'s> ParamValue<'s> {
    pub fn get_value_as_str(&'s self) -> &'s str {
        if self.value[0] == b'"' {
            std::str::from_utf8(&self.value[1..self.value.len() - 1]).unwrap()
        } else {
            std::str::from_utf8(self.value).unwrap()
        }
    }

    pub fn get_value<TResult: FromStr>(&'s self) -> TResult {
        let value = self.get_value_as_str();
        match TResult::from_str(value) {
            Ok(result) => result,
            Err(_) => panic!("Can not parse from string value: {}", value),
        }
    }
}
