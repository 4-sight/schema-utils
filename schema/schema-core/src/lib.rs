mod tests;

use proc_macro2::TokenStream;
use quote::quote;
use shared::{get_field_data, get_map_macro_token, get_object_token, get_value_token, FieldData};
use syn::{parse2, ItemStruct};

pub fn schema_core(input: TokenStream) -> TokenStream {
    let input_struct = match parse2::<ItemStruct>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    //==========================================================
    let schema_ident = &input_struct.ident;
    let object_token = get_object_token();
    let value_token = get_value_token();
    let map_macro_token = get_map_macro_token();
    let fields = get_field_data(&input_struct);

    //===============================================================
    // impl From<Schema> for Object...

    let impl_from = {
        let mappers: Vec<TokenStream> = fields
            .iter()
            .map(
                |FieldData {
                     f_ident,
                     f_name,
                     f_type_ident,
                     ..
                 }| {
                    if &f_type_ident.to_string() == "Vec" {
                        return quote! {
                            #f_name.into() => record.#f_ident.into_iter().map(|i| -> #value_token { i.into()}).collect::<Vec<#value_token>>().into()
                        }
                    }


                    quote! {
                        #f_name.into() => record.#f_ident.into()
                    }
                },
            )
            .collect();

        quote! {
            impl From<#schema_ident> for #object_token {
                fn from(record: #schema_ident) -> Self {
                    #map_macro_token!(
                        #(#mappers),*
                    ).into()
                }
            }
        }
    };

    //===============================================================
    // impl TryFrom<Object> for Schema...

    let impl_try_from = {
        let mappers: Vec<TokenStream> = fields
            .iter()
            .map(
                |FieldData {
                     f_ident, f_name, ..
                 }| {
                    quote! {#f_ident: schema_utils::get_x_from_object(&obj, #f_name.into())?}
                },
            )
            .collect();

        quote! {
            impl TryFrom<#object_token> for #schema_ident {
                type Error = schema_utils::Error;
                fn try_from(obj: #object_token) -> std::result::Result<#schema_ident, Self::Error> {
                    Ok(#schema_ident {
                        #(#mappers),*
                    })
                }
            }
        }
    };

    //===============================================================
    // output ========================================

    quote! {
        #impl_from
        #impl_try_from
    }
}
