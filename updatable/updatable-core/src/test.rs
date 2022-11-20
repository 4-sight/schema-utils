#![cfg(test)]

use crate::updatable_core;
use quote::quote;
use shared::assert_tokens_contain;

#[test]
fn update_struct_fields() {
    let before = quote! {
      pub struct Test {
        pub name: String,
        pub age: i64,
        pub pets: Vec<String>,
        pub job: Option<String>
      }
    };

    let expected = quote! {
      #[derive(Default)]
      pub struct TestUpdateInput {
        pub name: std::option::Option<String>,
        pub age: std::option::Option<i64>,
        pub pets: std::option::Option<Vec<String> >,
        pub job: std::option::Option<Option<String> >
      }
    };

    let after = updatable_core(before);
    assert_tokens_contain(&expected, &after);
}

#[test]
fn update_struct_impl() {
    let before = quote! {
      pub struct Test {
        pub name: String,
        pub age: i64,
        pub pets: Vec<String>,
        pub job: Option<String>
      }
    };

    let expected = quote! {
      impl TestUpdateInput {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn name(&mut self, val: String) -> &mut Self {
          self.name = std::option::Option::Some(val);
          self
        }

        pub fn age(&mut self, val: i64) -> &mut Self {
          self.age = std::option::Option::Some(val);
          self
        }

        pub fn pets(&mut self, val: Vec<String>) -> &mut Self {
          self.pets = std::option::Option::Some(val);
          self
        }

        pub fn job(&mut self, val: Option<String>) -> &mut Self {
          self.job = std::option::Option::Some(val);
          self
        }
    }
    };

    let after = updatable_core(before);
    assert_tokens_contain(&expected, &after);
}
