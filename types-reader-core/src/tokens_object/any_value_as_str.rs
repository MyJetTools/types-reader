use crate::MaybeEmptyValue;

pub trait AnyValueAsStr<'s> {
    fn try_as_str(&'s self) -> MaybeEmptyValue<&'s str>;
    fn as_str(&'s self) -> Result<&'s str, syn::Error>;
    fn throw_error(&self, message: &str) -> syn::Error;
}

impl<'s> TryInto<&'s str> for &'s dyn AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        match self.try_as_str() {
            MaybeEmptyValue::Empty => Err(self.throw_error("Value is empty")),
            MaybeEmptyValue::WithValue(value) => Ok(value),
        }
    }
}

impl<'s> TryInto<String> for &'s dyn AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self.try_as_str() {
            MaybeEmptyValue::Empty => Err(self.throw_error("Value is empty")),
            MaybeEmptyValue::WithValue(value) => Ok(value.to_string()),
        }
    }
}

impl<'s> TryInto<MaybeEmptyValue<&'s str>> for &'s dyn AnyValueAsStr<'s> {
    type Error = syn::Error;

    fn try_into(self) -> Result<MaybeEmptyValue<&'s str>, Self::Error> {
        Ok(self.try_as_str())
    }
}
