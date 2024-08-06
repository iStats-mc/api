use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn api_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    
    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        #[serde(rename_all = "camelCase")]
        #input
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn api_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    
    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        #input
    };
    
    TokenStream::from(expanded)
}