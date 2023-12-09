//used to import the proc macro crate, a rust a library 
// to implement procedural macros via provided types and functions
extern crate proc_macro;


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn}; //parser for rust syntax to parse the code into a syntax tree

#[proc_macro_attribute]
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream {
    
    let mut item_fn = parse_macro_input!(item as ItemFn);
    
    let ident = &item_fn.sig.ident;
    
    item_fn.block.stmts.insert(
        0,
        syn::parse_quote!(println!("Entering function: {}", stringify!(#ident));
        ),
    );
    
    TokenStream::from(quote! {
        #item_fn
    })
}
