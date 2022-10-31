use macros_utils::attributes::Attributes;

pub fn parse(src: &[syn::Attribute]) -> Attributes {
    let mut result = Attributes::new();
    for attr in src {
        for segment in attr.path.segments.iter() {
            let attr_id = segment.ident.to_string();
            let attr_data = attr.tokens.to_string();
            let attr_data = if attr_data == "" {
                None
            } else {
                Some(attr_data.into_bytes())
            };

            result.add(attr_id.to_string(), attr_data);
        }
    }

    result
}
