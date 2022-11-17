mod test;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shared::{get_field_data, get_map_macro_token, get_object_token, get_value_token, FieldData};
use syn::{parse2, ItemStruct};

pub fn updatable_core(input: TokenStream) -> TokenStream {
    let input_struct = match parse2::<ItemStruct>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let schema_ident = &input_struct.ident;
    let fields = get_field_data(&input_struct);
    let schema_update_ident = format_ident!("{}UpdateInput", schema_ident);

    let struct_field_tokens: Vec<TokenStream> = {
        fields
            .iter()
            .map(
                |FieldData {
                     f_ident,
                     f_type_path,
                     ..
                 }| {
                    quote! { pub #f_ident: std::option::Option<#f_type_path>}
                },
            )
            .collect()
    };

    let struct_impl_field_setters: Vec<TokenStream> = {
        fields
            .iter()
            .map(
                |FieldData {
                     f_ident,
                     f_type_path,
                     ..
                 }| {
                    quote! { pub fn #f_ident(&mut self, val: #f_type_path) -> &mut Self {
                            self.#f_ident = std::option::Option::Some(val);
                            self
                        }
                    }
                },
            )
            .collect()
    };

    quote! {
        #[derive(Default)]
        pub struct #schema_update_ident {
            #(#struct_field_tokens),*
        }

        impl #schema_update_ident {
            pub fn new() -> Self {
                Self::default()
            }

            #(#struct_impl_field_setters)*
        }
    }
}
