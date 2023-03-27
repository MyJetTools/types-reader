use std::str::FromStr;

use proc_macro2::TokenStream;

#[derive(Clone, Debug)]
pub struct ParamValue<'s> {
    pub value: &'s [u8],
    pub token: Option<&'s TokenStream>,
    pub ident: Option<&'s syn::Ident>,
}

impl<'s> ParamValue<'s> {
    pub fn as_str(&'s self) -> &'s str {
        if self.value.len() == 0 {
            return "";
        }

        if self.value[0] == b'"' {
            std::str::from_utf8(&self.value[1..self.value.len() - 1]).unwrap()
        } else {
            std::str::from_utf8(self.value).unwrap()
        }
    }

    pub fn get_value<TResult: FromStr>(
        &'s self,
        err_message: Option<&'static str>,
    ) -> Result<TResult, syn::Error> {
        let value = self.as_str();
        match TResult::from_str(value) {
            Ok(result) => Ok(result),
            Err(_) => {
                if let Some(err) = err_message {
                    return Err(self.to_err(format!("{}", err)));
                } else {
                    return Err(self.to_err(format!("Can not parse from string value: {}", value)));
                }
            }
        }
    }

    pub fn get_bool_value(&'s self) -> Result<bool, syn::Error> {
        let value = self.as_str().to_lowercase();
        match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                return Err(self.to_err(format!(
                    "Value must be 'true' or 'false'. Found value: {}",
                    value
                )));
            }
        }
    }

    pub fn to_err(&self, msg: String) -> syn::Error {
        if let Some(token) = self.token {
            return syn::Error::new_spanned(token, msg);
        }

        if let Some(ident) = self.ident {
            return syn::Error::new_spanned(ident, msg);
        }

        panic!("{}", msg)
    }
}
