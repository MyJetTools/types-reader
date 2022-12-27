use syn::TypePath;

use super::AsStr;
use quote::quote;

pub const U8: &str = "u8";
pub const I8: &str = "i8";
pub const U16: &str = "u16";
pub const I16: &str = "i16";
pub const U32: &str = "u32";
pub const I32: &str = "i32";
pub const U64: &str = "u64";
pub const I64: &str = "i64";
pub const F32: &str = "f32";
pub const F64: &str = "f64";
pub const U_SIZE: &str = "usize";
pub const I_SIZE: &str = "isize";
pub const BOOL: &str = "bool";
pub const STRING: &str = "String";
pub const STR: &str = "&str";
pub const DATETIME: &str = "DateTimeAsMicroseconds";

pub enum PropertyType<'s> {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    USize,
    ISize,
    String,
    Str,
    Bool,
    DateTime,
    OptionOf(Box<PropertyType<'s>>),
    VecOf(Box<PropertyType<'s>>),
    Struct(String, &'s TypePath),
}

impl<'s> PropertyType<'s> {
    pub fn new(field: &'s syn::Field) -> Self {
        match &field.ty {
            syn::Type::Slice(_) => panic!("Slice type is not supported"),
            syn::Type::Array(_) => panic!("Array type is not supported"),
            syn::Type::Ptr(_) => panic!("Ptr type is not supported"),
            syn::Type::Reference(_) => PropertyType::Str,
            syn::Type::BareFn(_) => panic!("BareFn type is not supported"),
            syn::Type::Never(_) => panic!("Never type is not supported"),
            syn::Type::Tuple(_) => panic!("Tuple type is not supported"),
            syn::Type::Path(type_path) => {
                let type_as_string = super::utils::simple_type_to_string(type_path);
                return PropertyType::parse(type_as_string, type_path);
            }
            syn::Type::TraitObject(_) => panic!("TraitObject type is not supported"),
            syn::Type::ImplTrait(_) => panic!("ImplTrait type is not supported"),
            syn::Type::Paren(_) => panic!("Paren type is not supported"),
            syn::Type::Group(_) => panic!("Group type is not supported"),
            syn::Type::Infer(_) => panic!("Infer type is not supported"),
            syn::Type::Macro(_) => panic!("Macro type is not supported"),
            syn::Type::Verbatim(_) => panic!("Verbatim type is not supported"),
            _ => panic!("{:?} type is not supported", &field.ty),
        }
    }

    pub fn parse(src: String, type_path: &'s TypePath) -> Self {
        match src.as_str() {
            U8 => PropertyType::U8,
            I8 => PropertyType::I8,
            U16 => PropertyType::U16,
            I16 => PropertyType::I16,
            U32 => PropertyType::U32,
            I32 => PropertyType::I32,
            U64 => PropertyType::U64,
            I64 => PropertyType::I64,
            F32 => PropertyType::F32,
            F64 => PropertyType::F64,
            U_SIZE => PropertyType::USize,
            I_SIZE => PropertyType::ISize,
            BOOL => PropertyType::Bool,
            STRING => PropertyType::String,
            DATETIME => PropertyType::DateTime,
            "Option" => PropertyType::OptionOf(Box::new(super::utils::get_generic(type_path))),
            "Vec" => PropertyType::VecOf(Box::new(super::utils::get_generic(type_path))),
            _ => PropertyType::Struct(src, type_path),
        }
    }

    pub fn as_str(&self) -> AsStr {
        match self {
            PropertyType::U8 => AsStr::create_as_str(U8),
            PropertyType::I8 => AsStr::create_as_str(I8),
            PropertyType::U16 => AsStr::create_as_str(U16),
            PropertyType::I16 => AsStr::create_as_str(I16),
            PropertyType::U32 => AsStr::create_as_str(U32),
            PropertyType::I32 => AsStr::create_as_str(I32),
            PropertyType::U64 => AsStr::create_as_str(U64),
            PropertyType::I64 => AsStr::create_as_str(I64),
            PropertyType::F32 => AsStr::create_as_str(F32),
            PropertyType::F64 => AsStr::create_as_str(F64),
            PropertyType::USize => AsStr::create_as_str(U_SIZE),
            PropertyType::ISize => AsStr::create_as_str(I_SIZE),
            PropertyType::String => AsStr::create_as_str(STRING),
            PropertyType::Str => AsStr::create_as_str(STR),
            PropertyType::Bool => AsStr::create_as_str(BOOL),
            PropertyType::DateTime => AsStr::create_as_str(DATETIME),

            PropertyType::OptionOf(generic_type) => {
                AsStr::create_as_string(format!("Option::<{}>", generic_type.as_str()))
            }
            PropertyType::VecOf(generic_type) => {
                AsStr::create_as_string(format!("Vec::<{}>", generic_type.as_str()))
            }
            PropertyType::Struct(_, type_path) => {
                panic!(
                    "Struct type is not supported in as_str method: {:?}",
                    type_path
                )
            }
        }
    }

    pub fn is_simple_type(&self) -> bool {
        match self {
            PropertyType::U8 => true,
            PropertyType::I8 => true,
            PropertyType::U16 => true,
            PropertyType::I16 => true,
            PropertyType::U32 => true,
            PropertyType::I32 => true,
            PropertyType::U64 => true,
            PropertyType::I64 => true,
            PropertyType::F64 => true,
            PropertyType::F32 => true,
            PropertyType::USize => true,
            PropertyType::ISize => true,
            PropertyType::String => true,
            PropertyType::Str => false,
            PropertyType::Bool => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        if let PropertyType::String = self {
            return true;
        }

        false
    }

    pub fn is_struct(&self) -> bool {
        if let PropertyType::Struct(_, _) = self {
            return true;
        }

        false
    }

    pub fn is_boolean(&self) -> bool {
        if let PropertyType::Bool = self {
            return true;
        }

        false
    }

    pub fn is_option(&self) -> bool {
        if let PropertyType::OptionOf(_) = self {
            return true;
        }

        false
    }

    pub fn is_u8(&self) -> bool {
        if let PropertyType::U8 = self {
            return true;
        }

        false
    }

    pub fn is_date_time(&self) -> bool {
        if let PropertyType::DateTime = self {
            return true;
        }

        false
    }

    pub fn get_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            PropertyType::U8 => quote!(u8),
            PropertyType::I8 => quote!(i8),
            PropertyType::U16 => quote!(u16),
            PropertyType::I16 => quote!(i16),
            PropertyType::U32 => quote!(u32),
            PropertyType::I32 => quote!(i32),
            PropertyType::U64 => quote!(u64),
            PropertyType::I64 => quote!(i64),
            PropertyType::F32 => quote!(f32),
            PropertyType::F64 => quote!(f64),
            PropertyType::USize => quote!(usize),
            PropertyType::ISize => quote!(isize),
            PropertyType::String => quote!(String),
            PropertyType::Str => todo!("get_token_stream Str is not supported"),
            PropertyType::Bool => quote!(bool),
            PropertyType::DateTime => quote!(rust_extensions::date_time::DateTimeAsMicroseconds),
            PropertyType::OptionOf(sub_type) => {
                let sub_type = sub_type.get_token_stream();
                quote!(Option::<#sub_type>)
            }
            PropertyType::VecOf(sub_type) => {
                let sub_type = sub_type.get_token_stream();
                quote!(Vec::<#sub_type>)
            }
            PropertyType::Struct(_, _) => todo!("get_token_stream Struct is not supported"),
        }
    }
}
