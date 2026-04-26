extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;

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

                    let hash1 = #ident_lower::digest([10u8; 50]);
                    let hash2 = #ident_lower::digest("Lorem ipsum");

                    // Length correctness
                    assert!(hash1.len() == #con::USIZE * 2);
                    assert!(hash2.len() == #con::USIZE * 2);

                    // Hashes digested different data
                    assert!(hash1 != hash2);
                }
            },
        ),
        (
            "array_convert",
            quote! {
                #[test]
                fn array_convert() {
                    use digest::typenum::Unsigned;

                    let array = [21u8; #con::USIZE];
                    let hybrid_array: digest::array::Array<u8, #con> = array.into();
                    let generic_array: digest::array::Array<u8, #con> = array.into();

                    let hash1: #ident_lower = array.into();
                    let hash2: #ident_lower = hybrid_array.into();
                    let hash3: #ident_lower = generic_array.into();
                    let hash4: #ident_lower = array.as_slice().try_into().unwrap();

                    // Length correctness
                    assert!(hash1.len() == #con::USIZE * 2);

                    // Use a different lib to test encoding
                    let encode = base16ct::lower::encode_string(&array);
                    assert!(hash1.as_bytes() == encode.as_bytes());

                    // All hashes should be the same
                    assert_eq!(hash1, hash2);
                    assert_eq!(hash2, hash3);
                    assert_eq!(hash3, hash4);
                }
            },
        ),
        (
            "string_convert",
            quote! {
                #[test]
                fn string_convert() {
                    use digest::typenum::Unsigned;

                    // Create a string padded using 'b' with the expected length (valid hex)
                    let mut string = "".to_string();
                    for i in 0..#con::USIZE * 2 {
                        string.push('b');
                    }

                    let str = string.as_str();
                    let array: [u8; _] = *string.as_bytes().as_array().unwrap();

                    let hash1: #ident_lower = string.as_str().try_into().unwrap();
                    let hash2: #ident_lower = string.clone().try_into().unwrap();
                    let hash3: #ident_lower = #ident_lower::from_hex(str).unwrap();
                    let hash4: #ident_lower = unsafe { #ident_lower::from_hex_unchecked(array) };

                    // Length correctness
                    assert!(hash1.len() == #con::USIZE * 2);

                    // Ensure round-trip
                    assert!(hash1.as_str() == str);
                    assert!(hash1.to_string() == string);

                    // All hashes should be the same
                    assert_eq!(hash1, hash2);
                    assert_eq!(hash2, hash3);
                    assert_eq!(hash3, hash4);
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
