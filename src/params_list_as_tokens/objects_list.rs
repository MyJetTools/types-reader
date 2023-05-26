use proc_macro2::TokenStream;

use crate::ParamsListAsTokens;

pub struct ObjectsList {
    pub objects: Vec<ParamsListAsTokens>,
}

impl ObjectsList {
    pub fn new(token_stream: TokenStream) -> Result<Self, syn::Error> {
        let mut objects = Vec::new();
        for itm in token_stream {
            match itm {
                proc_macro2::TokenTree::Group(group) => {
                    if let proc_macro2::Delimiter::Brace = group.delimiter() {
                        objects.push(ParamsListAsTokens::new(group.stream())?);
                    } else {
                        return Err(syn::Error::new_spanned(group, "Expected group of objects"));
                    }
                }
                _ => return Err(syn::Error::new_spanned(itm, "Expected group")),
            }
        }

        Ok(Self { objects })
    }
}
