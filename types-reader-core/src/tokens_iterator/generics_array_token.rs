use crate::{LifeTimeToken, TokensReader, TokensTreeExt};

pub struct GenericsArrayToken {
    content: Vec<LifeTimeToken>,
}

impl GenericsArrayToken {
    pub fn new(tokens_reader: &mut TokensReader) -> Result<Self, syn::Error> {
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

            let next_token = next_token.unwrap_as_punct_char();

            if next_token.is_none() {
                return Err(
                    tokens_reader.throw_error("Expected next token after open generics bracket")
                );
            }

            match next_token.unwrap() {
                '\'' => {
                    let life_time = LifeTimeToken::new(tokens_reader)?;
                    content.push(life_time);
                }
                '>' => {
                    tokens_reader.read_next_token()?;
                    break;
                }
                ',' => {
                    tokens_reader.read_next_token()?;
                }
                _ => {
                    return Err(syn::Error::new_spanned(next_token, "Invalid token"));
                }
            }
        }

        Ok(Self { content })
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

        quote::quote!(<#(#inners)*>)
    }
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

        let token = GenericsArrayToken::new(&mut tokens_reader).unwrap();

        assert_eq!("< 'a >", token.to_token_stream().to_string())
    }

    #[test]
    fn test_generic_with_two_lifetimes() {
        let src = proc_macro2::TokenStream::from_str("<'a, 'b>").unwrap();
        let mut tokens_reader = TokensReader::new(src);

        let token = GenericsArrayToken::new(&mut tokens_reader).unwrap();

        assert_eq!("< 'a , 'b >", token.to_token_stream().to_string())
    }
}
