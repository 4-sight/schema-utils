use proc_macro2::TokenStream;

pub fn assert_tokens_contain(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if !&actual.contains(&expected) {
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected pattern not found");
    }
}
