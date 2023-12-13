use proc_macro::TokenStream;
use types_reader_core::{PropertyType, StructProperty, StructureSchema};

pub const OBJECT_VALUE_TYPE_NAME: &str = "ObjectValue";
pub const TOKENS_OBJECT_TYPE_NAME: &str = "TokensObject";
pub const OPTIONAL_OBJECT_VALUE_TYPE_NAME: &str = "OptionalObjectValue";
pub const MAYBE_EMPTY_VALUE_TYPE_NAME: &str = "MaybeEmptyValue";
pub const ANY_VALUE_TYPE_NAME: &str = "AnyValue";

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let structure_schema = StructureSchema::new(&ast)?;
    generate_content(&structure_schema).map(Into::into)
}

pub fn generate_content(
    structure_schema: &StructureSchema,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut reading_props = Vec::new();

    for property in structure_schema.get_all() {
        let prop_ident = property.get_field_name_ident();

        let prop_name = prop_ident.to_string();

        reading_props.push(quote::quote!(#prop_ident: ));

        //todo!("Temporary reading ident and is_any_value_as_string is the same");
        let ident_is_allowed = super::utils::is_ident_allowed(property)
            || super::utils::is_any_value_as_string(property);

        let has_attribute = property.attrs.has_attr("has_attribute");

        if has_attribute {
            if !property.ty.is_boolean() {
                return property
                    .throw_error("'has_attribute' can be applied only to bool property");
            } else {
                reading_props.push(quote::quote! {
                    value.has_param(#prop_name),
                });
            }

            continue;
        }

        let is_default = super::utils::is_default(property);

        if property.ty.is_vec() {
            reading_props.push(generate_reading_from_vec(&prop_name));
        } else if let PropertyType::OptionOf(sub_ty) = &property.ty {
            reading_props.push(generate_reading_op(
                is_default,
                &prop_name,
                sub_ty,
                ident_is_allowed,
            ));
        } else {
            reading_props.push(read_param(
                &prop_name,
                property,
                ident_is_allowed,
                is_default,
            ));
        }
    }

    let name_ident = structure_schema.name.get_name_ident();

    let from_tokens_object = structure_schema.name.render_try_from_implementation(
        true,
        quote::quote!(types_reader::TokensObject),
        quote::quote!(syn::Error),
        || {
            quote::quote! {
                #name_ident::check_fields(value)?;
                    let result = Self{
                        #( #reading_props )*
                    };
                    Ok(result)
            }
        },
    );

    let check_fields = structure_schema.name.render_implement(|| {
        let mut add_fields = Vec::new();

        for field in structure_schema.get_all() {
            let name = field.name.as_str();
            add_fields.push(quote::quote! { #name, });
        }

        quote::quote! {

            pub fn check_fields(tokens_object: &types_reader::TokensObject)->Result<(), syn::Error>{
                tokens_object.check_for_unknown_params(&[#( #add_fields )*])
            }

        }
    });

    let result = quote::quote! {
        #from_tokens_object

        #check_fields
    };

    Ok(result)
}

fn generate_reading_op(
    reading_single_param: bool,
    prop_name: &str,
    sub_ty: &PropertyType,
    indent_is_allowed: bool,
) -> proc_macro2::TokenStream {
    if let PropertyType::RefTo { ty, lifetime: _ } = sub_ty {
        match ty.as_str().as_str() {
            TOKENS_OBJECT_TYPE_NAME => {
                return quote::quote! {
                     value.try_get_named_param(#prop_name),
                };
            }
            OPTIONAL_OBJECT_VALUE_TYPE_NAME => {
                if reading_single_param {
                    return quote::quote! {
                         value.try_get_value_from_single_or_named(#prop_name)?,
                    };
                } else {
                    return quote::quote! {
                         value.try_get_named_param(#prop_name),
                    };
                }
            }
            _ => {}
        }
    }

    if sub_ty.is_vec() {
        return quote::quote! {
            if let Some(value) = value.try_get_named_param(#prop_name){

                let items = value.unwrap_as_vec()?;
                let mut result = Vec::new();

                for item in items {
                    result.push(item.try_into()?);
                }

                Some(result)

            }else{
                None
            },
        };
    }

    if sub_ty.as_str().as_str() == MAYBE_EMPTY_VALUE_TYPE_NAME
        || sub_ty.as_str().as_str() == ANY_VALUE_TYPE_NAME
    {
        let any_value_as_string = if indent_is_allowed {
            quote::quote!(.unwrap_any_value_as_str()?)
        } else {
            quote::quote!()
        };

        return quote::quote! {
            if let Some(value) = value.try_get_named_param(#prop_name){
                Some(value.unwrap_as_value()? #any_value_as_string .try_into()?)
            }else{
                None
            },

        };
    }

    let reading_part = if indent_is_allowed {
        quote::quote! {
            Some(value.unwrap_any_value_as_str()?.try_into()?)
        }
    } else {
        quote::quote! {
                Some(value.try_into()?)
        }
    };

    if reading_single_param {
        return quote::quote! {

            if let Some(value) = value.try_get_value_from_single_or_named(#prop_name)?{
                if value.has_no_value(){
                    None
                }else{
                    #reading_part
                }
            }else{
                None
            },

        };
    } else {
        //                let value = value.unwrap_as_value()?;
        return quote::quote! {
            if let Some(value) = value.try_get_named_param(#prop_name){


                if value.has_no_value(){
                    None
                }else{
                    #reading_part
                }

            }else{
                None
            },

        };
    }
}

fn generate_reading_from_vec(prop_name: &str) -> proc_macro2::TokenStream {
    quote::quote!({
        {
            let mut result = Vec::new();
            let items = value.get_named_param(#prop_name)?.unwrap_as_vec()?;

            for item in items {
                result.push(item.try_into()?);
            }

            result
        }
    },)
}

fn read_param(
    prop_name: &str,
    property: &StructProperty,
    ident_is_allowed: bool,
    default: bool,
) -> proc_macro2::TokenStream {
    if let PropertyType::RefTo { ty, .. } = &property.ty {
        match ty.as_str().as_str() {
            TOKENS_OBJECT_TYPE_NAME => {
                return quote::quote! {
                     value.get_named_param(#prop_name)?,
                };
            }
            OBJECT_VALUE_TYPE_NAME => {
                if default {
                    return quote::quote! {
                         value.get_value_from_single_or_named(#prop_name)?
                         .try_into()?,
                    };
                } else {
                    return quote::quote! {
                         value.get_named_param(#prop_name)?
                         .unwrap_as_value()?
                         .try_into()?,
                    };
                }
            }

            OPTIONAL_OBJECT_VALUE_TYPE_NAME => {
                if default {
                    return quote::quote! {
                         value.get_value_from_single_or_named(#prop_name)?,
                    };
                } else {
                    return quote::quote! {
                         value.get_named_param(#prop_name)?
                         .unwrap_as_value()?,
                    };
                }
            }

            _ => {}
        }
    }

    let ty_str = property.ty.as_str();

    if ty_str.as_str() == MAYBE_EMPTY_VALUE_TYPE_NAME || ty_str.as_str() == ANY_VALUE_TYPE_NAME {
        let any_value_as_string = if ident_is_allowed {
            quote::quote!(.unwrap_any_value_as_str()?)
        } else {
            quote::quote!()
        };

        if default {
            return quote::quote! {
                 value.get_value_from_single_or_named(#prop_name)? #any_value_as_string  .try_into()?,
            };
        } else {
            return quote::quote! {
                 value.get_named_param(#prop_name)?
                 .unwrap_as_value()? #any_value_as_string  .try_into()?,
            };
        }
    }

    if default {
        if ident_is_allowed {
            return quote::quote! {
              value.get_value_from_single_or_named(#prop_name)?.unwrap_any_value_as_str()?.try_into()?,
            };
        } else {
            return quote::quote! {
              value.get_value_from_single_or_named(#prop_name)?.try_into()?,
            };
        }
    } else {
        if ident_is_allowed {
            return quote::quote! {
              value.get_named_param(#prop_name)?.unwrap_as_value()?.unwrap_any_value_as_str()?.try_into()?,
            };
        } else {
            return quote::quote! {
              value.get_named_param(#prop_name)?.try_into()?,
            };
        }
    }
}
