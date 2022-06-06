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

// use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, parse::Parser};
// use quote::quote;

#[proc_macro_attribute]
pub fn signature(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(syn::Field::parse_named.parse2(quote! {id: String}).unwrap());
                    fields.named.push(syn::Field::parse_named.parse2(quote! {name: String}).unwrap());
                    fields.named.push(syn::Field::parse_named.parse2(quote! {lifetime: usize}).unwrap());
                }   
                _ => {
                    ()
                }
            }
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`signature` has to be used with structs "),
    }
}