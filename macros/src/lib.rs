use proc_macro::TokenStream;
use quote::quote;
#[proc_macro]
pub fn sol_macro(_input:TokenStream) -> TokenStream{
    quote!{
        struct Solution{

        }
    }.into()
}