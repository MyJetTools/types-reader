use quote::ToTokens;

use super::{AttrParamsParser, ParamValue, SrcString};

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

pub enum ParamsType<'s> {
    None {
        attr_id: &'s syn::Ident,
        attr: &'s syn::Attribute,
    },
    Single {
        pos: Position,
        attr_id: &'s syn::Ident,
        attr: &'s syn::Attribute,
    },
    Multiple {
        pos: Vec<(Position, Position)>,
        attr_id: &'s syn::Ident,
        attr: &'s syn::Attribute,
    },
}

impl<'s> ParamsType<'s> {
    pub fn get_id_token(&self) -> &syn::Ident {
        match self {
            ParamsType::None { attr_id, .. } => attr_id,
            ParamsType::Single { attr_id, .. } => attr_id,
            ParamsType::Multiple { attr_id, .. } => attr_id,
        }
    }
}

pub struct AttributeParams<'s> {
    src: SrcString,
    pub param_type: ParamsType<'s>,
}

impl<'s> AttributeParams<'s> {
    pub fn new(attr: &'s syn::Attribute) -> Result<Self, syn::Error> {
        let a = attr.to_token_stream().to_string();

        println!("Attribute: {}", a);

        panic!("Implementing");

        /*
        for segment in &attr.path().segments {
            let attr_id = &segment.ident;
            let params = attr.bracket_token.to_string();

            if params == "" {
                return Ok(Self {
                    src: SrcString::new(params),
                    param_type: ParamsType::None { attr, attr_id },
                });
            }

            let src = SrcString::new(params);

            if let Some(pos) = is_single_value(src.get_str()) {
                return Ok(Self {
                    src,
                    param_type: ParamsType::Single { pos, attr, attr_id },
                });
            }
            return Ok(Self {
                param_type: ParamsType::Multiple {
                    pos: AttrParamsParser::new(src.get_str().as_bytes()).collect(),
                    attr,
                    attr_id,
                },
                src,
            });
        }

        Err(syn::Error::new_spanned(
            attr,
            "Attribute has wrong content to parse",
        ))
         */
    }

    pub fn get_single_param(&'s self) -> Result<ParamValue<'s>, syn::Error> {
        match &self.param_type {
            ParamsType::None { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_id_token(),
                "Attribute has no params",
            )),
            ParamsType::Single { pos, .. } => Ok(ParamValue {
                value: self.src.get_str()[pos.from..pos.to].as_bytes(),
            }),
            ParamsType::Multiple { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_id_token(),
                "Attribute has multiple params",
            )),
        }
    }

    pub fn get_named_param(&'s self, param_name: &str) -> Result<ParamValue<'s>, syn::Error> {
        match &self.param_type {
            ParamsType::None { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_id_token(),
                format!("Attribute has no params"),
            )),
            ParamsType::Single { .. } => Err(syn::Error::new_spanned(
                self.param_type.get_id_token(),
                format!("Attribute has single param"),
            )),
            ParamsType::Multiple { pos, .. } => {
                for (key, value) in pos {
                    let key = key.get_str(&self.src.get_str());

                    if key == param_name {
                        return Ok(ParamValue {
                            value: value.get_str(&self.src.get_str()).as_bytes(),
                        });
                    }
                }

                Err(syn::Error::new_spanned(
                    self.param_type.get_id_token(),
                    format!("Attribute has no param with name {}", param_name),
                ))
            }
        }
    }

    pub fn has_param(&self, param_name: &str) -> bool {
        if let ParamsType::Multiple { pos, .. } = &self.param_type {
            for (key, _) in pos {
                if key.get_str(&self.src.get_str()) == param_name {
                    return true;
                }
            }

            return false;
        }

        false
    }

    pub fn get_from_single_or_named(
        &'s self,
        param_name: &str,
    ) -> Result<ParamValue<'s>, syn::Error> {
        if let Ok(result) = self.get_single_param() {
            return Ok(result);
        }

        self.get_named_param(param_name)
    }

    pub fn get_attr_token(&'s self) -> &'s syn::Attribute {
        match self.param_type {
            ParamsType::None { attr, .. } => attr,
            ParamsType::Single { attr, .. } => attr,
            ParamsType::Multiple { attr, .. } => attr,
        }
    }

    pub fn get_id_token(&'s self) -> &'s syn::Ident {
        match self.param_type {
            ParamsType::None { attr_id, .. } => attr_id,
            ParamsType::Single { attr_id, .. } => attr_id,
            ParamsType::Multiple { attr_id, .. } => attr_id,
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
