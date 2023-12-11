use types_reader_core as types_reader;
use types_reader_core::MaybeEmptyValue;
use types_reader_macros::MacrosParameters;

#[derive(MacrosParameters)]
pub struct MyModel<'s> {
    pub a: MaybeEmptyValue<&'s str>,
    pub a_opt: Option<MaybeEmptyValue<&'s str>>,

    #[any_value_as_string]
    pub b: MaybeEmptyValue<&'s str>,
    #[any_value_as_string]
    pub b_opt: Option<MaybeEmptyValue<&'s str>>,
}
