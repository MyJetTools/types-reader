use crate::{LifeTimeToken, TokensReader, TokensTreeExt};

pub enum GenericItem {
    LifeTime(LifeTimeToken),
    Raw(proc_macro2::TokenStream),
}

impl GenericItem {
    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            GenericItem::LifeTime(life_time) => life_time.to_token_stream(),
            GenericItem::Raw(raw) => raw.clone(),
        }
    }
}

pub struct GenericsArrayToken {
    content: Vec<GenericItem>,
}

impl GenericsArrayToken {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    pub fn from_tokens_reader(tokens_reader: &mut TokensReader) -> Result<Self, syn::Error> {
        let next_token =
            tokens_reader.get_next_token(None, "Reading First Literal token failed")?;

        let open_bracket = next_token.unwrap_as_punct()?;

        if open_bracket.as_char() != '<' {
            return Err(syn::Error::new_spanned(
                open_bracket,
                "Expected open generics bracket",
            ));
        }

        let next_token =
            tokens_reader.peek_next_token("Expected next token after open generics bracket")?;

        if let Some(char) = next_token.unwrap_as_punct_char() {
            if char != '\'' {
                return Err(
                    tokens_reader.throw_error("Expected next token after open generics bracket")
                );
            }
        } else {
            return Err(
                tokens_reader.throw_error("Expected next token after open generics bracket")
            );
        }

        let mut content = Vec::new();

        loop {
            let next_token = tokens_reader.peek_next_token("Expecting some token going on")?;

            match next_token {
                crate::PeekedToken::Ident => {
                    let token = read_raw(tokens_reader)?;
                    content.push(GenericItem::Raw(token));
                }
                crate::PeekedToken::Punct(punct) => match punct {
                    '\'' => {
                        let life_time = LifeTimeToken::new(tokens_reader)?;
                        content.push(GenericItem::LifeTime(life_time));
                    }
                    '>' => {
                        tokens_reader.read_next_token()?;
                        break;
                    }
                    ',' => {
                        tokens_reader.read_next_token()?;
                    }
                    _ => {
                        let next_token = tokens_reader.read_next_token()?;
                        return Err(next_token.throw_error("Invalid token"));
                    }
                },
                crate::PeekedToken::Literal => {
                    return Err(tokens_reader.throw_error("Unexpected literal"));
                }

                crate::PeekedToken::Group(_) => {
                    return Err(tokens_reader.throw_error("Unexpected group"));
                }
            }
        }

        Ok(Self { content })
    }

    fn has_life_time(&mut self, name: &str) -> bool {
        for itm in &self.content {
            if let GenericItem::LifeTime(life_time) = itm {
                if life_time.as_str() == name {
                    return true;
                }
            }
        }
        false
    }

    pub fn add_life_time_if_not_exists(&mut self, life_time: &LifeTimeToken) {
        if !self.has_life_time(life_time.as_str()) {
            self.content.push(GenericItem::LifeTime(life_time.clone()));
        }
    }

    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let mut inners = Vec::new();

        for (i, c) in self.content.iter().enumerate() {
            let c = c.to_token_stream();

            if i < self.content.len() - 1 {
                inners.push(quote::quote!(#c,));
            } else {
                inners.push(quote::quote!(#c));
            }
        }

        if inners.len() == 0 {
            return quote::quote!();
        }
        quote::quote!(<#(#inners)*>)
    }

    pub fn get_first_life_time(&self) -> Option<&LifeTimeToken> {
        for itm in &self.content {
            if let GenericItem::LifeTime(life_time) = itm {
                return Some(life_time);
            }
        }
        None
    }
}

fn read_raw(tokens_reader: &mut TokensReader) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut raw = Vec::new();

    loop {
        let next_token = tokens_reader.peek_next_token("Expecting some token going on")?;

        if let Some(symbol) = next_token.unwrap_as_punct_char() {
            if symbol == ',' {
                break;
            }
            if symbol == '>' {
                break;
            }
        }

        let next_token = tokens_reader.read_next_token()?;
        raw.push(next_token.to_token_stream());
    }

    return Ok(quote::quote!(#(#raw)*));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::TokensReader;

    #[test]
    fn test_generic_with_single_lifetime() {
        let src = proc_macro2::TokenStream::from_str("<'a>").unwrap();
        let mut tokens_reader = TokensReader::new(src);

        let token = GenericsArrayToken::from_tokens_reader(&mut tokens_reader).unwrap();

        assert_eq!("< 'a >", token.to_token_stream().to_string())
    }

    #[test]
    fn test_generic_with_two_lifetimes() {
        let src = proc_macro2::TokenStream::from_str("<'a, 'b>").unwrap();
        let mut tokens_reader = TokensReader::new(src);

        let token = GenericsArrayToken::from_tokens_reader(&mut tokens_reader).unwrap();

        assert_eq!("< 'a , 'b >", token.to_token_stream().to_string())
    }

    #[test]
    fn test_generic_with_lifetime_and_type() {
        let src = proc_macro2::TokenStream::from_str("<'a, MyStructure>").unwrap();
        let mut tokens_reader = TokensReader::new(src);

        let token = GenericsArrayToken::from_tokens_reader(&mut tokens_reader).unwrap();

        assert_eq!("< 'a , MyStructure >", token.to_token_stream().to_string())
    }
}
