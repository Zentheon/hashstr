extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;

/// Generates impls for the simplest of hashers that don't require anything special
pub fn impl_hash_string_tests(input: TokenStream) -> TokenStream {
    let args: crate::Args = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let con = args.unwrap_con();
    let hasher = args.unwrap_hasher();
    let ident_lower = args.unwrap_ident_lower();
    let ident_upper = args.unwrap_ident_upper();

    let mod_name = format!("{}_tests", hasher.to_string().to_lowercase());
    let mod_ident = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());

    // Individually mapped tests, so they can be selectively ignored by the macro call
    let tests = vec![
        (
            "digest",
            quote! {
                #[test]
                fn digest() {
                    use digest::typenum::Unsigned;

                    let hash1 = #ident_lower::digest([30]);
                    assert!(hash1.0.len() == #con::USIZE * 2);
                }
            },
        ),
        (
            "array_convert",
            quote! {
                #[test]
                fn array_convert() {
                    use digest::typenum::Unsigned;

                    let array: [u8; _] = [21; #con::USIZE];
                    let hybrid_array: digest::array::Array<u8, #con> = array.into();
                    let generic_array: digest::array::Array<u8, #con> = array.into();

                    let hash1: #ident_lower = hybrid_array.into();
                    let hash2: #ident_lower = generic_array.into();
                    assert!(hash1.0.len() == #con::USIZE * 2);
                }
            },
        ),
        (
            "string_convert",
            quote! {
                #[test]
                fn string_convert() {
                    use digest::typenum::Unsigned;

                    let mut string = "".to_string();
                    for i in 0..#con::USIZE * 2 {
                        string.push('b');
                    }

                    let hash1: #ident_lower = string.as_str().try_into().unwrap();
                    let hash2: #ident_lower = string.try_into().unwrap();

                    assert!(hash1.0.len() == #con::USIZE * 2);
                }
            },
        ),
    ];
    // Parse the tests to include using a provided ignore list
    let mut tests_stream = quote!();
    let mut known_names: Vec<&str> = vec![];
    for (name, stream) in tests {
        known_names.push(name);

        if !args
            .ignore_tests
            .iter()
            .find(|s| s.value() == name)
            .is_some()
        {
            tests_stream.extend(stream);
        }
    }
    // Throw an error if a test in the ignored list isn't real
    for ignored in args.ignore_tests {
        if !known_names.contains(&ignored.value().as_str()) {
            abort!(ignored, "Test '{}' does not exist", ignored.value())
        }
    }

    // Finally insert the joined tests together into a dedicated module
    quote! {
        #[cfg(test)]
        mod #mod_ident {
            use super::*;
            #tests_stream
        }
    }
    .into()
}
