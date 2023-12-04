use types_reader_macros::MacrosEnum;

use types_reader_core as types_reader;

#[derive(MacrosEnum, Debug)]
pub enum MyEnum {
    Case1,
    #[default]
    Case2,
}
#[cfg(test)]
mod tests {
    use crate::test_enums::test_default_case::MyEnum;

    #[test]
    fn test_default() {
        assert_eq!("Case2", format!("{:?}", MyEnum::default()));
    }
}
