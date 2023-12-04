use types_reader::AnyValueAsStr;
use types_reader_core as types_reader;

//#[derive(MacrosParameters)]
pub struct HttpActionResult<'s> {
    pub status_code: u16,
    pub description: &'s str,
    //#[allow_ident]
    pub model: Option<&'s str>,
}

impl<'s> TryInto<HttpActionResult<'s>> for &'s types_reader::TokensObject {
    type Error = syn::Error;
    fn try_into(self) -> Result<HttpActionResult<'s>, Self::Error> {
        let result = HttpActionResult {
            status_code: self.get_named_param("status_code")?.try_into()?,
            description: self.get_named_param("description")?.try_into()?,
            model: if let Some(value) = self.try_get_named_param("model") {
                Some(value.get_value()?.any_value_as_str().as_str())
            } else {
                None
            },
        };
        Ok(result)
    }
}
