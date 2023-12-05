use crate::{type_name::TypeName, StructProperty};

pub struct StructureSchema<'s> {
    properties: Vec<StructProperty<'s>>,
    pub name: TypeName,
}

impl<'s> StructureSchema<'s> {
    pub fn new(data: &'s syn::DeriveInput) -> Result<Self, syn::Error> {
        let properties = StructProperty::read(&data).unwrap();
        let result = Self {
            properties,
            name: TypeName::from_derive_input(data)?,
        };

        Ok(result)
    }

    pub fn remove(&'s mut self, name: &str) -> Option<StructProperty<'s>> {
        let mut index = None;

        for i in 0..self.properties.len() {
            if self.properties[i].name == name {
                index = Some(i);
                break;
            }
        }

        let index = index?;

        let result = self.properties.remove(index);
        Some(result)
    }

    pub fn get_all(&'s self) -> &'s [StructProperty<'s>] {
        &self.properties
    }
}
