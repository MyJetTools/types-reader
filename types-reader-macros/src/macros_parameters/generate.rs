use proc_macro::TokenStream;
use types_reader_core::StructureSchema;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let structure_schema = StructureSchema::new(&ast);

    let mut reading_props = Vec::new();

    for property in structure_schema.get_all() {
        let prop_ident = property.get_field_name_ident();
        reading_props.push(quote::quote! {
            #prop_ident: tokens_reader.get_value()?.try_into()?,
        });
    }

    let result = structure_schema.render_try_into_implementation(
        quote::quote!(types_reader::ObjectValue),
        quote::quote!(syn::Error),
        || {
            quote::quote! {
                pub fn new(src: proc_macro::TokenStream)->Result<Self, syn::Error>{
                    use types_reader::*;

                    let src:proc_macro2::TokenStream = src.into();

                    let mut tokens_reader = TokensObject::new(src.into())?;

                    let result = Self{
                      #( #reading_props )*
                    };

                    Ok(result)
                }

            }
        },
    );

    Ok(result.into())
}
