#![cfg(test)]

use crate::schema_core;
use quote::quote;
use shared::assert_tokens_contain;

#[test]
fn impl_from_schema_for_object() {
    let before = quote! {
      pub struct Test {
        pub name: String,
        pub age: i64,
        pub pets: Vec<String>,
        pub job: Option<String>
      }
    };

    let expected = quote! {
      impl From<Test> for surrealdb::sql::Object {
        fn from(record: Test) -> Self {
          schema_utils::map!(
            "name".into() => record.name.into(),
            "age".into() => record.age.into(),
            "pets".into() => record.pets.into_iter().map(|i| -> surrealdb::sql::Value {i.into()}).collect::<Vec<surrealdb::sql::Value>>().into(),
            "job".into() => record.job.into()
          ).into()
        }
      }
    };

    let after = schema_core(before);
    assert_tokens_contain(&expected, &after);
}

#[test]
fn impl_try_from_object_for_schema() {
    let before = quote! {
      pub struct Test {
        pub name: String,
        pub age: i64,
        pub pets: Vec<String>,
        pub job: Option<String>
      }
    };

    let expected = quote! {
      impl TryFrom<surrealdb::sql::Object> for Test {
        type Error = schema_utils::Error;
        fn try_from(obj: surrealdb::sql::Object) -> std::result::Result<Test, Self::Error> {
          Ok(Test {
              name: schema_utils::get_x_from_object(&obj, "name".into())?,
              age: schema_utils::get_x_from_object(&obj, "age".into())?,
              pets: schema_utils::get_x_from_object(&obj, "pets".into())?,
              job: schema_utils::get_x_from_object(&obj, "job".into())?
            }
          )
        }
      }
    };

    let after = schema_core(before);
    assert_tokens_contain(&expected, &after);
}
