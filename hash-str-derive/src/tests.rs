extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::quote;

use crate::ident;

pub fn impl_hash_str_tests(input: TokenStream) -> TokenStream {
    let args: crate::Args = match syn::parse(input) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let ident = args.struct_ident(args.upper);
    let ident_lower = args.struct_ident(false);
    let ident_upper = args.struct_ident(true);
    let hash_name = args.hash_name_str();

    let upper = args.upper;
    let con = args.unwrap_con();

    let mod_ident = ident!("{}_tests", args.struct_snake(args.upper));

    // Individually mapped tests, so they can be selectively ignored by the macro call
    let tests = vec![
        (
            "digest",
            quote! {
                #[test]
                fn digest() {
                    use digest::typenum::Unsigned;

                    let hash1 = #ident::digest([10u8; 50]);
                    let hash2 = #ident::digest("Lorem ipsum");

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

                    let hash1: #ident = array.into();
                    let hash2: #ident = hybrid_array.into();
                    let hash3: #ident = generic_array.into();
                    let hash4: #ident = array.as_slice().try_into().unwrap();

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
                    let char = if #upper {'B'} else {'b'};
                    let mut b_string = "".to_string();
                    for i in 0..#con::USIZE * 2 {
                        b_string.push(char);
                    }

                    let array: [u8; _] = *b_string.as_bytes().as_array().unwrap();

                    let hash1: #ident = b_string.as_str().try_into().unwrap();
                    let hash2: #ident = b_string.clone().try_into().unwrap();
                    let hash3: #ident = #ident::from_hex(b_string.as_str()).unwrap();
                    let hash4: #ident = unsafe { #ident::from_hex_unchecked(array) };

                    // Debug print
                    println!("input string (len {}): {b_string}", b_string.len());
                    println!("hash1 (len {}): {hash1:?}", hash1.len());
                    println!("hash2 (len {}): {hash2:?}", hash2.len());
                    println!("hash3 (len {}): {hash3:?}", hash3.len());
                    println!("hash4 (len {}): {hash4:?}", hash4.len());

                    // Length correctness
                    assert!(hash1.len() == #con::USIZE * 2);

                    // Ensure round-trip
                    assert!(hash1.as_str() == b_string.as_str());
                    assert!(hash1.to_string() == b_string);

                    // All hashes should be the same
                    assert_eq!(hash1, hash2);
                    assert_eq!(hash2, hash3);
                    assert_eq!(hash3, hash4);
                }
            },
        ),
        (
            "casing",
            quote! {
                #[test]
                fn casing() {
                    use digest::typenum::Unsigned;

                    // Data vars
                    let data_str = "jfkdlasjfie;ajld;nfsdnajejfu3827ura8pdfsL>Vmzx,./vM3/43j2k";
                    let array = [187u8; #con::USIZE]; // Unencoded `bb`
                    let hex_lower = [98u8; #con::USIZE * 2]; // lowercase 'b`
                    let hex_upper = [66u8; #con::USIZE * 2]; // Uppercase 'B'

                    // Lowercase hashes
                    let lower = #ident_lower::digest(data_str);
                    let lower_b1 = #ident_lower::from(array);
                    let lower_b2 = unsafe { #ident_lower::from_hex_unchecked(hex_lower) };

                    // Uppercase hashes
                    let upper = #ident_upper::digest(data_str);
                    let upper_b1 = #ident_upper::from(array);
                    let upper_b2 = unsafe { #ident_upper::from_hex_unchecked(hex_upper) };

                    // Debug print
                    println!("lower: {lower:?}");
                    println!("lower_b1: {lower_b1:?}");
                    println!("lower_b2: {lower_b2:?}");
                    println!("====");
                    println!("upper: {upper:?}");
                    println!("upper_b1: {upper_b1:?}");
                    println!("upper_b2: {upper_b2:?}");

                    // No `lower` hash should match `upper`
                    assert!(lower != upper);
                    assert!(lower_b1 != upper_b1);
                    assert!(lower_b2 != upper_b2);

                    // `upper` to lowercase should match `lower`...
                    assert!(lower == upper.to_lowercase());
                    assert!(lower_b1 == upper_b1.to_lowercase());
                    assert!(lower_b2 == upper_b2.to_lowercase());

                    // ...and vice versa
                    assert!(lower.to_uppercase() == upper);
                    assert!(lower_b1.to_uppercase() == upper_b1);
                    assert!(lower_b2.to_uppercase() == upper_b2);

                    // b1 and b2 hashes should match
                    assert!(lower_b1 == lower_b2);
                    assert!(upper_b1 == upper_b2);

                    // Assert lowercase is lowercase
                    let found = lower.find(|c: char| crate::HEX_LETTERS_UPPER.contains(&{ c as u8 }));
                    assert!(found.is_none());

                    // And finally, check uppercase
                    let found = upper.find(|c: char| crate::HEX_LETTERS_LOWER.contains(&{ c as u8 }));
                    assert!(found.is_none());
                }
            },
        ),
        (
            "length_error",
            quote! {
                #[test]
                fn length_error() {
                    use digest::typenum::Unsigned;
                    const N: usize = #con::USIZE;

                    let too_long = [20u8; N + 10];
                    let too_short = [20u8; N - 2];
                    let res1 = #ident::try_from(too_long.as_slice());
                    let res2 = #ident::encode_bytes(too_short.as_slice());

                    match res1 {
                        Err(crate::Error::LengthError(crate::LengthError {
                            expected,
                            actual,
                            hash_name,
                        })) => {
                            assert!(expected == N);
                            assert!(actual == N + 10);
                            assert!(hash_name == #hash_name);
                        },
                        _ => panic!("This should be a LengthError"),
                    };

                    match res2 {
                        Err(crate::Error::LengthError(crate::LengthError {
                            expected,
                            actual,
                            hash_name,
                        })) => {
                            assert!(expected == N);
                            assert!(actual == N - 2);
                            assert!(hash_name == #hash_name);
                        },
                        _ => panic!("This should be a LengthError"),
                    };
                }
            },
        ),
        (
            "hex_error",
            quote! {
                #[test]
                fn hex_error() {
                    use digest::typenum::Unsigned;
                    const HEX_LEN: usize = #con::USIZE * 2;

                    let oops_all_z = fstr::FStr::<HEX_LEN>::from_ascii_filler(b'z');
                    let mut slice = fstr::FStr::<HEX_LEN>::from_ascii_filler(b'a').into_bytes();
                    slice[HEX_LEN / 2] = 'g' as u8;
                    let single_bad_char = fstr::FStr::from_bytes(slice).unwrap();

                    let res1 = #ident::try_from(oops_all_z);
                    let res2 = #ident::from_hex(single_bad_char);

                    match res1 {
                        Err(crate::Error::HexError(crate::HexError {
                            char,
                            index,
                            hash_name,
                        })) => {
                            assert!(char == 'z');
                            assert!(index == 0);
                            assert!(hash_name == #hash_name);
                        },
                        _ => panic!("This should be a HexError"),
                    };

                    match res2 {
                        Err(crate::Error::HexError(crate::HexError {
                            char,
                            index,
                            hash_name,
                        })) => {
                            assert!(char == 'g');
                            assert!(index == HEX_LEN / 2);
                            assert!(hash_name == #hash_name);
                        },
                        _ => panic!("This should be a HexError"),
                    };
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
