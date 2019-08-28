extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AsConcrete)]
pub fn as_concrete(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    impl_as_concrete(&input)
}

fn impl_as_concrete(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AsConcrete<#name> for #name {
            fn as_concrete(&self) -> #name {
                self.clone()
            }
        }
    };
    gen.into()
}