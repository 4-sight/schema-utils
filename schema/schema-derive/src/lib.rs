use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use schema_core::schema_core;

#[proc_macro_error]
#[proc_macro_derive(Schema)]
pub fn schema(input: TokenStream) -> TokenStream {
    schema_core(input.into()).into()
}
