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
        .get_value_from_single_or_named("name")?
        .as_string()?
        .as_str();

    let ident_with_generics = structure_schema.name.to_token_stream();

    let impl_generics = structure_schema.name.get_generic_token_stream_after_impl();

    Ok(quote::quote! {
        #ast
        impl #impl_generics types_reader::MacrosAttribute for #ident_with_generics {
            const NAME:&'static str = #attribute_name;
        }
    }
    .into())
}
