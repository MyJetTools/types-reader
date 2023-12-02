mod macros_parameters;

use proc_macro::TokenStream;

#[proc_macro_derive(MacrosParameters, attributes(http_enum_case))]
pub fn my_http_integer_enum_derive(input: TokenStream) -> TokenStream {
    match crate::macros_parameters::generate(input) {
        Ok(result) => result,
        Err(err) => err.to_compile_error().into(),
    }
}
