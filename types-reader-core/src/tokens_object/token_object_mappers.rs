use crate::TokensObject;

impl<'s> TryInto<&'s str> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<&'s str, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_string()?;
        Ok(result.as_str())
    }
}

impl<'s> TryInto<String> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let value = self.unwrap_as_value()?;
        let value = value.as_string()?;
        Ok(value.to_string())
    }
}

impl<'s> TryInto<bool> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        let value = self.unwrap_as_value()?;
        let value = value.as_bool()?.get_value();
        Ok(value)
    }
}

impl<'s> TryInto<i8> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<i8, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_i8();
        Ok(result)
    }
}

impl<'s> TryInto<u8> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_u8();
        Ok(result)
    }
}

impl<'s> TryInto<i16> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<i16, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_i16();
        Ok(result)
    }
}

impl<'s> TryInto<u16> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<u16, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_u16();
        Ok(result)
    }
}

impl<'s> TryInto<i32> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<i32, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_i32();
        Ok(result)
    }
}

impl<'s> TryInto<u32> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_u32();
        Ok(result)
    }
}

impl<'s> TryInto<i64> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_i64();
        Ok(result)
    }
}

impl<'s> TryInto<u64> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_u64();
        Ok(result)
    }
}

impl<'s> TryInto<isize> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<isize, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_isize();
        Ok(result)
    }
}

impl<'s> TryInto<usize> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<usize, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_number()?.as_usize();
        Ok(result)
    }
}

impl<'s> TryInto<f32> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<f32, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_double()?.as_f32();
        Ok(result)
    }
}

impl<'s> TryInto<f64> for &'s TokensObject {
    type Error = syn::Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        let value = self.unwrap_as_value()?;
        let result = value.as_double()?.as_f64();
        Ok(result)
    }
}
