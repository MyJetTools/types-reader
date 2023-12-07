use types_reader_core::StructProperty;

pub fn is_ident_allowed(case: &StructProperty) -> bool {
    case.attrs.try_get_attr("allow_ident").is_some()
}

pub fn is_any_value_as_string(case: &StructProperty) -> bool {
    case.attrs.try_get_attr("any_value_as_string").is_some()
}

pub fn is_default(case: &StructProperty) -> bool {
    case.attrs.try_get_attr("default").is_some()
}
