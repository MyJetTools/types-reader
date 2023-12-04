pub trait AnyValueAsStr<'s> {
    fn as_str(&'s self) -> &str;
    fn throw_error(&self, message: &str) -> syn::Error;
}

impl<'s> TryInto<&'s str> for &'s dyn AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        Ok(self.as_str())
    }
}

impl<'s> TryInto<String> for &'s dyn AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let value = self.as_str();
        Ok(value.to_string())
    }
}
