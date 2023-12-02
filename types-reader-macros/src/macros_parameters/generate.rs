use proc_macro::TokenStream;

pub fn generate(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let ident = &ast.ident;

    let result = quote::quote! {
        impl #ident{

        }
    };

    Ok(result.into())
}
