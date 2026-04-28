use digest::consts::U64;
use fstr::FStr;
use hash_strings_derive::impl_hash_string;
use whirlpool::Whirlpool;

impl_hash_string!(hasher = whirlpool::Whirlpool, con = U64);

#[test]
fn dings() {
    let fstr = unsafe { FStr::from_inner_unchecked([98u8; 128]) };
    let lower = WhirlpoolString(fstr);
    let upper = WhirlpoolStringUpper::try_from(&fstr).unwrap();

    println!("Lower: {lower:?}, upper: {upper:?}");
    println!("lower to uppercase: {}", lower.to_uppercase());
    println!("upper to uppercase: {}", upper.to_uppercase());
    assert!(lower != upper);
    assert!(lower == upper.to_lowercase());

    fstr.len();
    let char = char::from_u32(30).unwrap();
    let u8 = char as u8;

    let found = lower.find(|c: char| crate::HEX_LETTERS_LOWER.contains(&{ c as u8 }));
    // assert!(found.is_none());
}
