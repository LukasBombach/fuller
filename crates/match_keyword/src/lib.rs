extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::ExprMatch;

#[proc_macro]
pub fn match_keyword(input: TokenStream) -> TokenStream {
    eprintln!("INPUT: {:#?}", input);

    let syntax: ExprMatch = syn::parse(input).unwrap();

    // println!("{}", quote!(#syntax));

    let gen = quote! {
        12
    };
    gen.into()
}
