use proc_macro::TokenStream;
use types_reader_core::{StructureSchema, TokensObject};

pub fn generate(
    input: TokenStream,
    attr: proc_macro2::TokenStream,
) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let structure_schema = StructureSchema::new(&ast)?;

    let token_objects: TokensObject = attr.try_into()?;

    let attribute_name = token_objects
        .get_from_single_or_named("name")?
        .as_string()?
        .as_str();

    let parameters = crate::macros_parameters::generate_content(&structure_schema)?;

    let name_ident = structure_schema.name.get_name_ident();

    Ok(quote::quote! {
        impl #name_ident {
            pub fn get_attr_name()->&'static str{
                #attribute_name
            }
        }
        #parameters

    }
    .into())
}
