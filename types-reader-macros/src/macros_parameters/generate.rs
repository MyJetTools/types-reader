use proc_macro::TokenStream;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let ident = &ast.ident;

    let result = quote::quote! {
        impl #ident{

            pub fn new(src: proc_macro::TokenStream)->Result<Self, syn::Error>{
                use types_reader::*;

                let src:proc_macro2::TokenStream = src.into();

                let params_list = TokensObject::new(src.into(), &||None)?;

                todo!("Implement")
            }

        }
    };

    Ok(result.into())
}
