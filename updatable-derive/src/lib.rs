use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use updatable_core::updatable_core;

#[proc_macro_error]
#[proc_macro_derive(Updatable)]
pub fn updatable(input: TokenStream) -> TokenStream {
    updatable_core(input.into()).into()
}
