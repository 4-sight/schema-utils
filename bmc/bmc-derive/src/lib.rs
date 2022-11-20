use bmc_core::*;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(Bmc, attributes(entity))]
pub fn bmc_derive(input: TokenStream) -> TokenStream {
	bmc_core(input.into()).into()
}
