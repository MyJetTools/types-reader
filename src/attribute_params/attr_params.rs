use proc_macro2::TokenStream;
use quote::ToTokens;

use super::{AttrParamsParser, ParamValue};

pub struct Position {
    pub from: usize,
    pub to: usize,
}

impl Position {
    pub fn get_str<'s>(&self, src: &'s str) -> &'s str {
        if self.to == 0 {
            return "";
        }

        &src[self.from..self.to]
    }
}

pub enum ParamsType {
    None {
        attr: TokenStream,
    },
    Single {
        pos: Position,
        attr: TokenStream,
    },
    Multiple {
        pos: Vec<(Position, Position)>,
        attr: TokenStream,
    },
}

impl ParamsType {
    pub fn get_attr_token(&self) -> &TokenStream {
        match self {
            ParamsType::None { attr, .. } => attr,
            ParamsType::Single { attr, .. } => attr,
            ParamsType::Multiple { attr, .. } => attr,
        }
    }
}

pub struct AttributeParams {
    src: String,
    pub param_type: ParamsType,
}

impl AttributeParams {
    pub fn new(attr: &syn::Attribute) -> Result<Self, syn::Error> {
        let attributes = attr.to_token_stream().to_string();

        if !attributes.starts_with("#") {
            return Err(syn::Error::new_spanned(
                attr,
                "Attribute has to start with #",
            ));
        }

        let params = super::attr_parse_utils::find_params(&attributes[1..]);

        Self::create(attr.to_token_stream(), params)
    }

    pub fn from_token_string(token_stream: TokenStream) -> Result<Self, syn::Error> {
        let as_string = token_stream.to_string();

        Self::create(token_stream, Some(as_string))
    }

    fn create(attr: TokenStream, params: Option<String>) -> Result<Self, syn::Error> {
        match params {
            Some(params) => {
                if let Some(pos) = is_single_value(&params) {
                    return Ok(Self {
                        src: params.to_string(),
                        param_type: ParamsType::Single { pos, attr },
                    });
                }
                return Ok(Self {
                    param_type: ParamsType::Multiple {
                        pos: AttrParamsParser::new(params.as_bytes()).collect(),
                        attr,
                    },
                    src: params.to_string(),
                });
            }
            None => {
                return Ok(Self {
                    src: "".to_string(),
                    param_type: ParamsType::None { attr },
                });
            }
        }
    }

    pub fn get_single_param<'s>(&'s self) -> Result<ParamValue<'s>, syn::Error> {
        match &self.param_type {
            ParamsType::None { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_attr_token(),
                "Attribute has no params",
            )),
            ParamsType::Single { pos, .. } => Ok(ParamValue {
                value: self.src[pos.from..pos.to].as_bytes(),
            }),
            ParamsType::Multiple { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_attr_token(),
                "Attribute has multiple params",
            )),
        }
    }

    pub fn get_named_param<'s>(&'s self, param_name: &str) -> Result<ParamValue<'s>, syn::Error> {
        match &self.param_type {
            ParamsType::None { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_attr_token(),
                format!("Attribute has no params"),
            )),
            ParamsType::Single { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_attr_token(),
                format!("Attribute has single param"),
            )),
            ParamsType::Multiple { pos, .. } => {
                for (key, value) in pos {
                    let key = key.get_str(&self.src.as_str());

                    if key == param_name {
                        return Ok(ParamValue {
                            value: value.get_str(&self.src.as_str()).as_bytes(),
                        });
                    }
                }

                Err(syn::Error::new_spanned(
                    self.param_type.get_attr_token(),
                    format!("Attribute has no param with name {}", param_name),
                ))
            }
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        if let ParamsType::Multiple { pos, .. } = &self.param_type {
            for (key, _) in pos {
                if key.get_str(&self.src.as_str()) == param_name {
                    return true;
                }
            }

            return false;
        }

        false
    }

    pub fn get_from_single_or_named<'s>(
        &'s self,
        param_name: &str,
    ) -> Result<ParamValue<'s>, syn::Error> {
        if let Ok(result) = self.get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }

    pub fn get_attr_token<'s>(&'s self) -> &TokenStream {
        match &self.param_type {
            ParamsType::None { attr, .. } => attr,
            ParamsType::Single { attr, .. } => attr,
            ParamsType::Multiple { attr, .. } => attr,
        }
    }

    pub fn has_params_at_all(&self) -> bool {
        match self.param_type {
            ParamsType::None { .. } => false,
            ParamsType::Single { .. } => true,
            ParamsType::Multiple { .. } => true,
        }
    }
}

fn is_single_value(src: &str) -> Option<Position> {
    let src_as_bytes = src.as_bytes();
    if src_as_bytes[0] == b'"' {
        return Some(Position {
            from: 1,
            to: src.len() - 1,
        });
    }

    if !src_as_bytes.iter().any(|itm| *itm <= 32u8) {
        return Some(Position {
            from: 0,
            to: src.len(),
        });
    }

    None
}
