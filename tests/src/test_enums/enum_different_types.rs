use types_reader_core as types_reader;
use types_reader_macros::{MacrosEnum, MacrosParameters};

#[derive(MacrosEnum, Debug)]
pub enum ShouldBeAuthorized {
    Yes,
    No,
    YesWithClaims(Vec<String>),
}

#[derive(MacrosParameters)]
pub struct MyStruct {
    pub field1: ShouldBeAuthorized,
    #[has_attribute]
    pub as_str: bool,
    pub field2: Option<ShouldBeAuthorized>,
    pub deprecated: Option<bool>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use types_reader_core::TokensObject;

    use super::MyStruct;

    #[test]
    fn test_yes_string_case() {
        let params = r#"field1: "Yes""#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: MyStruct = (&tokens).try_into().unwrap();

        assert_eq!("Yes", format!("{:?}", my_struct.field1));
    }

    #[test]
    fn test_no_string_case() {
        let params = r#"field1: "No""#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: MyStruct = (&tokens).try_into().unwrap();

        assert_eq!("No", format!("{:?}", my_struct.field1));
    }

    #[test]
    fn test_vec_of_strings_case() {
        let params = r#"field1: ["Option1","Option2"]"#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: MyStruct = (&tokens).try_into().unwrap();

        assert_eq!(
            "YesWithClaims([\"Option1\", \"Option2\"])",
            format!("{:?}", my_struct.field1)
        );
    }

    #[test]
    fn test_yess_string_case() {
        let params = r#"field1: "Yess""#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: Result<MyStruct, syn::Error> = (&tokens).try_into();

        assert_eq!(my_struct.is_err(), true);
    }
}
