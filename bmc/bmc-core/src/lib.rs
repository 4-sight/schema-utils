mod tests;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote};
use shared::get_update_struct_ident;
use syn::{parse2, DeriveInput, Ident, Result};

pub fn bmc_core(input: TokenStream) -> TokenStream {
	let DeriveInput {
		ident: schema_ident,
		attrs,
		..
	} = match parse2(input) {
		Ok(syntax_tree) => syntax_tree,
		Err(error) => return error.to_compile_error(),
	};

	let bmc_struct_ident = format_ident!("{}Bmc", &schema_ident);
	let update_struct_ident = get_update_struct_ident(&schema_ident);
	let attribute = attrs
		.iter()
		.find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "entity");
	let parameters = attribute
		.map(|a| match parse2::<Params>(a.tokens.clone()) {
			Ok(params) => params,
			Err(e) => {
				println!("{:?}", e);
				abort!(a, "Invalid entity attribute")
			}
		})
		.unwrap_or_default();
	let entity = &parameters.entity.unwrap_or_else(|| schema_ident.to_string().to_lowercase());

	quote! {
		pub struct #bmc_struct_ident;

		impl #bmc_struct_ident {
			const ENTITY: &'static str = #entity;

			pub async fn get(ctx: std::sync::Arc<Ctx>, id:&str) -> 	Result<Record<#schema_ident>> {
				bmc_get::<#schema_ident>(ctx, Self::ENTITY, id).await
			}

			pub async fn create(ctx: std::sync::Arc<Ctx>, data:#schema_ident) -> 	Result<String> {
				bmc_create(ctx, Self::ENTITY, data.into()).await
			}

			pub async fn delete(ctx: std::sync::Arc<Ctx>, id:&str) -> 	Result<String> {
				bmc_delete(ctx, Self::ENTITY, id).await
			}

			pub async fn update(ctx: std::sync::Arc<Ctx>, id:&str, data: #update_struct_ident) -> 	Result<String> {
				bmc_update(ctx, Self::ENTITY, id, data.into()).await
			}

			pub async fn select(ctx: std::sync::Arc<Ctx>, filter: std::option::Option<String>, sort: std::option::Option<String>) -> Result<Vec<Record<#schema_ident>>> {
				bmc_select::<#schema_ident>(ctx, Self::ENTITY, filter, sort).await
			}
		}
	}
}

#[derive(Debug, Default)]
struct Params {
	entity: Option<String>,
}

impl syn::parse::Parse for Params {
	fn parse(input: syn::parse::ParseStream) -> Result<Self> {
		let content;
		syn::parenthesized!(content in input);
		let entity_ident: Ident = content.parse()?;

		Ok(Self {
			entity: Some(entity_ident.to_string()),
		})
	}
}
