use types_reader_core as types_reader;
use types_reader_macros::MacrosParameters;

#[derive(MacrosParameters)]
pub struct SubParameters<'s> {
    pub sub_parameter1: &'s str,
    pub sub_parameter2: &'s str,
}

#[derive(MacrosParameters)]
pub struct HttpActionResult<'s> {
    pub status_code: u16,
    pub description: SubParameters<'s>,
    #[allow_ident]
    pub model: Option<&'s str>,

    pub as_vec: Vec<SubParameters<'s>>,
    pub as_vec_opt: Option<Vec<SubParameters<'s>>>,
}
