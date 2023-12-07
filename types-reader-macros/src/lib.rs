mod attribute_name;
mod macros_enum;
mod macros_parameters;
use proc_macro::TokenStream;

#[proc_macro_derive(
    MacrosParameters,
    attributes(any_value_as_string, allow_ident, default, has_attribute)
)]
pub fn macros_parameters(input: TokenStream) -> TokenStream {
    match crate::macros_parameters::generate(input) {
        Ok(result) => result,
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(MacrosEnum, attributes(value, default))]
pub fn macros_enum(input: TokenStream) -> TokenStream {
    match crate::macros_enum::generate(input) {
        Ok(result) => result,
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn attribute_name(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match crate::attribute_name::generate(input, attr.into()) {
        Ok(result) => result,
        Err(err) => err.to_compile_error().into(),
    }
}
