pub struct EnumModel {}

impl EnumModel {
    pub fn new(variant: &syn::Variant) -> Option<Self> {
        println!("{:?}", variant);
        None
    }
}
