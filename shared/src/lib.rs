mod assert_tokens_contain;
mod error;
mod field_data;
mod get_x_from_object;
mod into_value;
mod map_macro;
mod parse_into;
mod try_from_i64;
mod try_from_value;
mod try_into_integer;

pub use assert_tokens_contain::assert_tokens_contain;
pub use error::Error;
pub use field_data::FieldData;
pub use get_x_from_object::*;
pub use into_value::IntoValue;
pub use parse_into::ParseInto;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort;
use quote::format_ident;
use syn::{parse_str, Field, Fields::Named, ItemStruct};
pub use try_from_i64::TryFromI64;
pub use try_from_value::*;
pub use try_into_integer::TryIntoInteger;

pub fn get_object_token() -> TokenStream {
    parse_str::<TokenStream>("surrealdb::sql::Object").unwrap()
}

pub fn get_value_token() -> TokenStream {
    parse_str::<TokenStream>("surrealdb::sql::Value").unwrap()
}

pub fn get_map_macro_token() -> TokenStream {
    parse_str::<TokenStream>("schema_utils::map").unwrap()
}

pub fn get_newtype_token() -> TokenStream {
    parse_str::<TokenStream>("schema_utils::NewTypeWrapper").unwrap()
}

pub fn get_named_fields(s: &ItemStruct) -> Vec<Field> {
    let Named(n) =  s.fields.clone() else {
      abort!(s, "Struct must contain named fields");
  };

    n.named.into_iter().collect()
}

pub fn get_field_data(s: &ItemStruct) -> Vec<FieldData> {
    get_named_fields(s).into_iter().map(|f| f.into()).collect()
}

pub fn get_update_struct_ident(input_ident: &Ident) -> Ident {
    format_ident!("{}UpdateInput", input_ident)
}
