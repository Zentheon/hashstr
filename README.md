# What is *hash-strings*
> Convenient string-like types that wrap the various [RustCrypto: Hashes](https://github.com/RustCrypto/hashes).

RustCrypto is a great project that offers up a bunch of Rusty hashing algorithms under one nice, unified API. One small problem: It requires moderate brain use to convert the types around in a portable manor. This simply cannot stand! Where is the Python-level convenience?

Enter `hash-strings`, a collection of crates that wrap their RustCrypto counterparts in types that behave as close to regular 'ol [`String`]s as possible but with the added benefit of content digesting-related methods and an additional coat of type enforcement paint.

# Usage
Pick out the crate and algorithm that suits your needs. We'll use [`sha3::Sha3_256`] for demonstration, but usage is unanimous across all `hash-strings` variants.

## Digesting
there are currently 3 methods available to quickly obtain a hash of some data:
```rs
let hash1 = Sha3_256String::digest(b"abc");

let mut reader = std::io::Cursor::new([200 % 44 + (45 ^ 6) - 25]);
let hash2 = Sha3_256String::digest_reader(&mut reader)?;

let hash3 = Sha3_256String::digest_file("./example.txt")?;
```

## Default
The [`Default`] impl for any given type is simply just:
```rs
fn default() -> Self {
    Self::digest([])
}
```
This results in the hash of digesting exactly nothing. In the case of [`sha3::Sha3_256`] specifically: `e3f209a895a66061878916ed4ca48bf2f8e6cf37de93a7d362f4db9b303688e0862d87e701d30b27956969ca6b9be398e889ca6c0bc681108f0d6fb57c2808e5`
