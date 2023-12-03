use proc_macro2::{Delimiter, TokenTree};

pub enum PeekedToken {
    Ident,
    Group(Delimiter),
    Punct(char),
    Literal,
}

impl PeekedToken {
    pub fn from(src: &TokenTree) -> Self {
        match src {
            TokenTree::Group(value) => Self::Group(value.delimiter()),
            TokenTree::Ident(_) => Self::Ident,
            TokenTree::Punct(value) => Self::Punct(value.as_char()),
            TokenTree::Literal(_) => Self::Literal,
        }
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident)
    }

    pub fn is_punct(&self) -> bool {
        matches!(self, Self::Punct(_))
    }

    pub fn unwrap_as_punct_char(&self) -> Option<char> {
        match self {
            Self::Punct(value) => Some(*value),
            _ => None,
        }
    }
}
