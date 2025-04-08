extern crate proc_macro; //declare external module
use proc_macro::TokenStream; //TokenStream type which represents a series of Rust syntax token
use syn::{parse_macro_input, DeriveInput}; //syn is a parsing lib for parsing a stream of Rust tokens
use quote::quote; //crate which provides quote! macro for turning Rust syntax tree data structures into tokens of src code

//implementation of derive for our own traits using the procedural macro library 
//traits implement methods which types can use

#[proc_macro_derive(MyCustomTrait)] //hey Rust, we are defining a proecedrual custom derive macro (generates code when applied to struc #[derive(MyCustomTrait)])
pub fn my_custom_trait(input: TokenStream) -> TokenStream { //when we use the #[derive()], Rust runs this method
    let input = parse_macro_input!(input as DeriveInput); //syn::Derive, parse input tokens into a syntax tree

    let name = input.ident; //get name of struc

    let expanded = quote! {//generate implementation of MyCustomTrait which will be used when we call the macro on a struc
        impl MyCustomTrait for #name {
            fn custom_method() {
                println!("Hello from MyCustomTrait for {}!", stringify!(#name));
            }
        }
    };

    //convert generated code back into TokenStream
    TokenStream::from(expanded)
}