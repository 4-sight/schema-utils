#![cfg(test)]

use crate::bmc_core;
use quote::quote;
use shared::assert_tokens_contain;

#[test]
fn bmc_struct() {
	let before = quote! {
	  pub struct Test {
		pub name: String,
		pub age: i64,
		pub pets: Vec<String>,
		pub job: Option<String>
	  }
	};

	let expected = quote! {
	  pub struct TestBmc;
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn default_entity() {
	let before = quote! {
	  pub struct Test {
		pub name: String,
		pub age: i64,
		pub pets: Vec<String>,
		pub job: Option<String>
	  }
	};

	let expected = quote! {
	  const ENTITY: &'static str = "test";
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn named_entity() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	  const ENTITY: &'static str = "custom";
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn test_get() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	  pub async fn get(ctx: std::sync::Arc<Ctx>, id:&str) -> Result<Record<Test>> {
	  bmc_get::<Test>(ctx, Self::ENTITY, id).await
	}
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn test_create() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	  pub async fn create(ctx: std::sync::Arc<Ctx>, data: Test) -> Result<String> {
	  bmc_create(ctx, Self::ENTITY, data.into()).await
	}
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn test_delete() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	  pub async fn delete(ctx: std::sync::Arc<Ctx>, id:&str) -> Result<String> {
	  bmc_delete(ctx, Self::ENTITY, id).await
	}
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn test_update() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	pub async fn update(ctx: std::sync::Arc<Ctx>, id:&str,  data: TestUpdateInput) -> Result<String> {
	  bmc_update(ctx, Self::ENTITY, id, data.into()).await
	}
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}

#[test]
fn test_select() {
	let before = quote! {
	#[entity(custom)]
	pub struct Test {
	pub name: String,
	pub age: i64,
	pub pets: Vec<String>,
	pub job: Option<String>
	}
	};

	let expected = quote! {
	pub async fn select(ctx: std::sync::Arc<Ctx>, filter: std::option::Option<String>, sort: std::option::Option<String>) -> Result<Vec<Record<Test>>> {
	  bmc_select::<Test>(ctx, Self::ENTITY, filter, sort).await
	}
	};

	let after = bmc_core(before);
	assert_tokens_contain(&expected, &after);
}
