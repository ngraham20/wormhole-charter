extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Signature)]
pub fn signature_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_signature_derive(&ast)
}

fn impl_signature_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Signature for #name {
            fn id(&self) -> String {
                self.id.clone()
            }
            fn name(&self) -> String {
                self.name.clone()
            }
            fn lifetime(&self) -> usize {
                self.lifetime
            }
        }
    };
    gen.into()
}