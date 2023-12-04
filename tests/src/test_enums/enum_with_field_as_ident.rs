use types_reader::AnyValueAsStr;
use types_reader_core as types_reader;
use types_reader_macros::{MacrosEnum, MacrosParameters};

#[derive(MacrosEnum, Debug)]
pub enum ShouldBeAuthorized {
    Yes,
    No,
}

#[derive(MacrosParameters)]
pub struct MyStruct {
    #[allow_ident]
    pub field1: ShouldBeAuthorized,
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
    fn test_maybe_string_case() {
        let params = r#"field1: "Maybe""#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: Result<MyStruct, syn::Error> = (&tokens).try_into();

        assert_eq!(my_struct.is_err(), true);
    }
}
