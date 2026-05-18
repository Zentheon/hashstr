# What is *hash-str*
> Convenient string-like types that wrap the various [RustCrypto: Hashes](https://github.com/RustCrypto/hashes).

RustCrypto is a great project that offers up a bunch of Rusty hashing algorithms under one nice, unified API. One small problem: It requires moderate brain use to convert the types around in a portable manor. This simply cannot stand! Where is the Python-level convenience?

Enter `hash-str`, a single crate that wraps most RustCrypto hashers in types that behave as close to regular 'ol [`String`]s as possible but with the added benefit of content digesting-related methods, improved efficiency and an additional coat of type enforcement paint.

# Usage
Pick out the algorithm that suits your needs. `hash-str` does not enable any hasher types by default, so we'll go with [`sha2`](https://docs.rs/sha2/latest) for demonstration, but usage is unanimous across ~~all~~ **most** variants.

Add to your Cargo.toml:
```toml
hash-str = { version = "0.1", features = ["sha2"] }
```
or run:
```bash
cargo add -F sha3 hash-str 
```

## Digest
there are currently 3 methods available to quickly obtain a hash of some data:
```rust
use hash_str::sha2::Sha256String;

fn main() -> std::io::Result<()> {
    let hash1 = Sha256String::digest(b"abc");
    println!("Hash of 'abc': {hash1}");
    
    let mut reader = std::io::Cursor::new([200 % 44 + (45 ^ 6) - 25]);
    let hash2 = Sha256String::digest_reader(&mut reader)?;
    println!("Hash of '42': {hash2}");
    
    #[cfg(no_run)] // This one is omitted from doctests 
    let hash3 = Sha256String::digest_file("./example.txt")?;
    
    assert!(hash1 != hash2);
    Ok(())
}
```

## Casing
Hash strings are lowercase by default. Every type has an `*Upper` variant:
```rust
use hash_str::sha2::{ Sha224String, Sha224StringUpper };

let data = b"Tasty data";

let hash_lower = Sha224String::digest(data);
let hash_upper = Sha224StringUpper::digest(data);

assert!(hash_lower == "62b0d0b4104870095c7e3bbfd470b828c4e9edfe46d26d04f9dc799f");
assert!(hash_upper == "62B0D0B4104870095C7E3BBFD470B828C4E9EDFE46D26D04F9DC799F");
```

## Conversion
Say you already used a hasher directly to obtain one of the somewhat cryptic const array outputs. Its `hash-str` counterpart implements `From` traits for them:
```rust
use hash_str::sha2::Sha512String;
use sha2::digest::Digest;

let mut hasher = sha2::Sha512::new();
hasher.update("There was a guy named Vlad...");
hasher.update([189, 147, 249]);

let hash: Sha512String = hasher.finalize().into();
```

## Default
The [`Default`] impl for any given type is simply just:
```rust,ignore
fn default() -> Self {
    Self::digest([])
}
```
This results in the hash of digesting exactly nothing. In the case of [`sha2::Sha256`](https://docs.rs/sha2/0.11.0/sha2/struct.Sha224.html) specifically...
```rust
use hash_str::sha2::Sha256String;

assert!(Sha256String::default() == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
```

## Efficiency
`hash-str`s aren't just convenient, they're efficient!

* All types wrap [fstr](https://docs.rs/fstr/0.2.18/fstr/) under the hood, which allows for fixed-sized [`str`]-likes. 
* Encoding is done with [`const_hex`](https://docs.rs/const-hex/latest)
* `PartialEq` impls use [constant_time_eq](https://docs.rs/constant_time_eq/latest) for best practice. 
* Casing conversion uses a purpose-built function.

# Limitations
The memory footprint of encoded-by-default hashes is an improvement compared to [`String`]s, but encoded hex is still twice the size of its raw representation. A truly astronomical amount of hashes would need to be handled for this to become a concern, though.

## XOF
Extendable output XOF hashers are currently unsupported. Examples include:
* [`sha3::Shake128`](https://docs.rs/sha3/latest/sha3/struct.Shake128.html)
* [`sha3::Shake256`](https://docs.rs/sha3/latest/sha3/struct.Shake256.html)
* [`turboshake`](https://docs.rs/turboshake/latest)
* [`cshake`](https://docs.rs/cshake/latest)

One exception to this is `blake3`.
