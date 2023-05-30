use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{ParamValue, ParamsList, VecOfValues};

pub fn get_list_of_elements(token_stream: TokenStream) -> Result<ParamValue, syn::Error> {
    let mut result = None;
    for itm in token_stream.clone() {
        match itm {
            proc_macro2::TokenTree::Group(group) => {
                if let proc_macro2::Delimiter::Brace = group.delimiter() {
                    if result.is_none() {
                        result = Some(ParamValue::ObjectList {
                            token_stream: group.clone().into_token_stream(),
                            value: Vec::new(),
                        })
                    }

                    let result = result.as_mut().unwrap();

                    match result {
                        ParamValue::ObjectList { value, .. } => {
                            value.push(ParamsList::new(group.stream())?)
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                group,
                                "Each array element must be an object",
                            ));
                        }
                    }
                } else {
                    return Err(syn::Error::new_spanned(group, "Expected group of objects"));
                }
            }
            proc_macro2::TokenTree::Punct(_) => {}
            proc_macro2::TokenTree::Literal(literal) => {
                if result.is_none() {
                    result = Some(ParamValue::VecOfValues(VecOfValues::new(
                        literal.clone().to_token_stream(),
                    )))
                }

                let result = result.as_mut().unwrap();

                match result {
                    ParamValue::VecOfValues(value) => {
                        value.add_value(literal);
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            literal,
                            "Each array element must be a string",
                        ));
                    }
                }
            }
            _ => return Err(syn::Error::new_spanned(itm, "Expected group")),
        }
    }

    if result.is_none() {
        return Err(syn::Error::new_spanned(
            token_stream,
            "Expected group of objects",
        ));
    }

    Ok(result.unwrap())
}
