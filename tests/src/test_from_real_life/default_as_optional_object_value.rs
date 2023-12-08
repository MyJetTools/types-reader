use types_reader::*;
use types_reader_core as types_reader;
use types_reader_macros::*;

#[attribute_name("default")]
#[derive(MacrosParameters, Clone)]
pub struct DefaultAttribute<'s> {
    #[default]
    pub value: &'s OptionalObjectValue,
}
