use proc_macro2::TokenTree;

pub trait TokensTreeExt {
    fn is_ident(&self) -> bool;
    fn is_punct(&self) -> bool;
    fn is_punct_with_char(&self, char: char) -> bool;

    fn unwrap_as_ident(self) -> Result<proc_macro2::Ident, syn::Error>;

    fn unwrap_as_literal(self) -> Result<proc_macro2::Literal, syn::Error>;

    fn unwrap_as_punct(self) -> Result<proc_macro2::Punct, syn::Error>;

    fn unwrap_as_group(self) -> Result<proc_macro2::Group, syn::Error>;

    fn throw_error<T>(&self, message: &str) -> Result<T, syn::Error>;
}

impl TokensTreeExt for TokenTree {
    fn is_ident(&self) -> bool {
        match self {
            TokenTree::Ident(_) => true,
            _ => false,
        }
    }

    fn is_punct(&self) -> bool {
        match self {
            TokenTree::Punct(_) => true,
            _ => false,
        }
    }

    fn is_punct_with_char(&self, char: char) -> bool {
        match self {
            TokenTree::Punct(punkt) => punkt.as_char() == char,
            _ => false,
        }
    }

    fn unwrap_as_ident(self) -> Result<proc_macro2::Ident, syn::Error> {
        match self {
            TokenTree::Ident(ident) => Ok(ident),
            _ => Err(syn::Error::new_spanned(self, "Expecting Ident here")),
        }
    }

    fn unwrap_as_group(self) -> Result<proc_macro2::Group, syn::Error> {
        match self {
            TokenTree::Group(group) => Ok(group),
            _ => Err(syn::Error::new_spanned(self, "Expecting Group here")),
        }
    }

    fn unwrap_as_literal(self) -> Result<proc_macro2::Literal, syn::Error> {
        match self {
            TokenTree::Literal(literal) => Ok(literal),
            _ => Err(syn::Error::new_spanned(self, "Expecting literal here")),
        }
    }

    fn unwrap_as_punct(self) -> Result<proc_macro2::Punct, syn::Error> {
        match self {
            TokenTree::Punct(punct) => Ok(punct),
            _ => Err(syn::Error::new_spanned(self, "Expecting punct here")),
        }
    }

    fn throw_error<T>(&self, message: &str) -> Result<T, syn::Error> {
        Err(syn::Error::new_spanned(self, message))
    }
}
