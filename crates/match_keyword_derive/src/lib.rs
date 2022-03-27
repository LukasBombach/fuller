extern crate proc_macro;

use proc_macro::TokenStream;
// use quote::quote;

#[proc_macro]
pub fn match_keyword(item: TokenStream) -> TokenStream {
    println!("item: \"{}\"", item.to_string());
    item
}

/* #[proc_macro]
pub fn match_keyword(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_match_keyword(&ast)
}

fn impl_match_keyword(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MatchKeyword for #name {
            fn match_keyword() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
} */

/* #[proc_macro_derive(MatchKeyword)]
pub fn match_keyword_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_match_keyword(&ast)
}

fn impl_match_keyword(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MatchKeyword for #name {
            fn match_keyword() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
} */
