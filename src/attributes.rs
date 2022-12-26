use std::collections::HashMap;

use macros_utils::AttributeParams;

pub fn parse(src: &[syn::Attribute]) -> HashMap<String, Option<AttributeParams>> {
    let mut result = HashMap::new();

    for attr in src {
        for segment in attr.path.segments.iter() {
            let attr_id = segment.ident.to_string();
            let attr_data = attr.tokens.to_string();
            if attr_data == "" {
                result.insert(attr_id.to_string(), None);
            } else {
                result.insert(attr_id.to_string(), Some(AttributeParams::new(attr_data)));
            }
        }
    }

    result
}
