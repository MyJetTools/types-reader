use rust_extensions::StrOrString;
use types_reader_core::EnumCase;

pub fn get_enum_str_value<'s>(case: &'s EnumCase) -> Result<StrOrString<'s>, syn::Error> {
    let value = case.attrs.try_get_attr("value");

    if value.is_none() {
        return Ok(case.get_name_ident().to_string().into());
    }

    let value = value.unwrap();

    let value = value.get_value_from_single_or_named("value")?;

    let value: &str = value.try_into()?;
    Ok(value.into())
}

pub fn has_default_attribute(case: &EnumCase) -> bool {
    case.attrs.try_get_attr("default").is_some()
}
