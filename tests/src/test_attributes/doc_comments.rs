use types_reader_core as types_reader;
use types_reader_macros::{MacrosEnum, MacrosParameters};

/// A doc comment on the container.
#[derive(MacrosParameters)]
pub struct StructWithDocComments {
    /// A doc comment on a field.
    pub card_number: Option<bool>,

    /// A doc comment on a field
    /// which is spread across several lines.
    #[has_attribute]
    pub as_str: bool,
}

/// A doc comment on the enum container.
#[derive(MacrosEnum, Debug)]
pub enum EnumWithDocComments {
    /// A doc comment on the enum case.
    Yes,
    /// Another doc comment on the enum case.
    No,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use types_reader_core::TokensObject;

    use super::*;

    #[test]
    fn test_struct_with_doc_comments_on_fields_is_readable() {
        let params = r#"card_number: true, as_str"#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let my_struct: StructWithDocComments = (&tokens).try_into().unwrap();

        assert_eq!(my_struct.card_number, Some(true));
        assert_eq!(my_struct.as_str, true);
    }

    #[test]
    fn test_enum_with_doc_comments_on_cases_is_readable() {
        let params = r#"field: "Yes""#;

        let params = proc_macro2::TokenStream::from_str(params).unwrap();

        let tokens: TokensObject = params.try_into().unwrap();

        let value = tokens.get_named_param("field").unwrap();

        let value: EnumWithDocComments = value.try_into().unwrap();

        assert_eq!("Yes", format!("{:?}", value));
    }
}
