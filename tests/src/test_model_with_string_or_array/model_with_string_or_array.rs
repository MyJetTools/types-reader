use types_reader::TokensObject;
use types_reader_core as types_reader;
use types_reader_macros::*;
#[derive(MacrosEnum)]
enum ShouldBeAuthorized {
    Yes,
    No,
    YesWithClaims(Vec<String>),
}

impl ShouldBeAuthorized {
    pub fn is_yes(&self) -> bool {
        matches!(self, Self::Yes)
    }

    pub fn is_no(&self) -> bool {
        matches!(self, Self::No)
    }

    pub fn unwrap_as_yes_with_claims(&self) -> &[String] {
        match { self } {
            Self::YesWithClaims(claims) => claims,
            _ => panic!("can not unwrap as YesWithClaims"),
        }
    }
}

#[derive(MacrosParameters)]
pub struct StructToTest {
    value: ShouldBeAuthorized,
}

#[test]
fn test_parse_as_yes_case() {
    let value = proc_macro2::TokenStream::from(quote::quote!(value: "Yes"));

    let value: TokensObject = value.try_into().unwrap();

    let result: StructToTest = (&value).try_into().unwrap();
    assert_eq!(true, result.value.is_yes());
}

#[test]
fn test_parse_as_no_case() {
    let value = proc_macro2::TokenStream::from(quote::quote!(value: "No"));

    let value: TokensObject = value.try_into().unwrap();

    let result: StructToTest = (&value).try_into().unwrap();
    assert_eq!(true, result.value.is_no());
}

#[test]
fn test_parse_as_array() {
    let value = proc_macro2::TokenStream::from(quote::quote!(value: ["MyValue"]));

    let value: TokensObject = value.try_into().unwrap();

    let result: StructToTest = (&value).try_into().unwrap();

    let value = result.value.unwrap_as_yes_with_claims();

    assert_eq!(1, value.len());

    assert_eq!(
        result.value.unwrap_as_yes_with_claims().get(0).unwrap(),
        "MyValue"
    );
}
