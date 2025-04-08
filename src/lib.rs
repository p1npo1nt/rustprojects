extern crate proc_macro; //declare external module
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput}; //syn is a parsing lib for parsing a stream of Rust tokens
use quote::quote; //crate which provides quote! macro for turning Rust syntax tree data structures into tokens of src code

//implementation of derive for our own traits using the procedural macro library 
//traits implement methods which types can use

#[proc_macro_derive(MyCustomTrait)]
pub fn my_custom_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput); //syn::Derive

    let name = input.ident; //get name of struc

    let expanded = quote! {//generate implementation of MyCustomTrait
        impl MyCustomTrait for #name {
            fn custom_method() {
                println!("Hello from MyCustomTrait for {}!", stringify!(#name));
            }
        }
    };

    //convert generated code back into TokenStream
    TokenStream::from(expanded)
}

pub trait MyCustomTrait {
    fn custom_method();
}