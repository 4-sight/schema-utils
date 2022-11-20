mod test;

use proc_macro2::TokenStream;
use quote::quote;
use shared::{
	get_field_data, get_newtype_token, get_object_token, get_update_struct_ident,
	get_value_token, FieldData,
};
use syn::{parse2, ItemStruct};

pub fn updatable_core(input: TokenStream) -> TokenStream {
	let input_struct = match parse2::<ItemStruct>(input) {
		Ok(syntax_tree) => syntax_tree,
		Err(error) => return error.to_compile_error(),
	};

	let schema_ident = &input_struct.ident;
	let fields = get_field_data(&input_struct);
	let schema_update_ident = get_update_struct_ident(schema_ident);
	let object_token = get_object_token();
	let value_token = get_value_token();
	let newtype_token = get_newtype_token();

	let struct_field_tokens: Vec<TokenStream> = {
		fields
			.iter()
			.map(|FieldData { f_ident, f_type_path, .. }| {
				quote! { pub #f_ident: std::option::Option<#f_type_path>}
			})
			.collect()
	};

	let struct_impl_field_setters: Vec<TokenStream> = {
		fields
			.iter()
			.map(|FieldData { f_ident, f_type_path, .. }| {
				quote! { pub fn #f_ident(&mut self, val: #f_type_path) -> &mut Self {
						self.#f_ident = std::option::Option::Some(val);
						self
					}
				}
			})
			.collect()
	};

	let object_from_update_field_mappers: Vec<TokenStream> = {
		fields
			.iter()
			.map(|FieldData { f_ident, f_name, .. }| {
				quote! { if let Some(#f_ident) = i.#f_ident {
					updates.insert(#f_name.into(), #newtype_token(#f_ident).into());
				}}
			})
			.collect()
	};

	quote! {
		// pub struct SchemaUpdateInput {..}
		#[derive(Default)]
		pub struct #schema_update_ident {
			#(#struct_field_tokens),*
		}

		// impl SchemaUpdateInput {..}
		impl #schema_update_ident {
			pub fn new() -> Self {
				Self::default()
			}

			#(#struct_impl_field_setters)*
		}

		// impl From<SchemaUpdateInput> for Object {..}
		impl From<#schema_update_ident> for #object_token {
			fn from(i: #schema_update_ident) -> #object_token {
				let mut updates = std::collections::BTreeMap::<String, #value_token>::new();
				#(#object_from_update_field_mappers)*

				updates.into()
			}
		}
	}
}
