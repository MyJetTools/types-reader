use types_reader_core as types_reader;
use types_reader_macros::*;

#[derive(MacrosParameters)]
pub struct HttpActionResult<'s> {
    #[allow_ident]
    #[default]
    pub model: Option<&'s str>,
}
