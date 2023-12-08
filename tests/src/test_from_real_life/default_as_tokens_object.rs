use types_reader::*;
use types_reader_core as types_reader;
use types_reader_macros::*;

#[attribute_name("default")]
#[derive(MacrosParameters, Clone)]
pub struct DefaultAttribute<'s> {
    pub value: Option<&'s TokensObject>,
    pub value2: &'s TokensObject,
}
