use blake2::{Blake2b512, Blake2s256, digest::generic_array::GenericArray};
use digest::consts::{U32, U64};
use hash_str_derive::impl_hash_str;

impl_hash_str!(
    hasher = blake2::Blake2s256,
    con = U32,
    digest = "
        use blake2::Digest;

        let mut hasher = Blake2s256::new();
        hasher.update(data.as_ref());
        let hash = Self::from(hasher.finalize());
    ",
    digest_reader = "
        use blake2::Digest;

        let mut hasher = Blake2s256::new();
        let digested = std::io::copy(read, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
    digest_file = "
        use blake2::Digest;

        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Blake2s256::new();
        let digested = std::io::copy(&mut file, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
);

impl_hash_str!(
    hasher = blake2::Blake2b512,
    con = U64,
    digest = "
    use blake2::Digest;

    let mut hasher = Blake2b512::new();
    hasher.update(data.as_ref());
    let hash = Self::from(hasher.finalize());
",
    digest_reader = "
    use blake2::Digest;

    let mut hasher = Blake2b512::new();
    let digested = std::io::copy(read, &mut hasher)?;
    let hash = Self::from(hasher.finalize());
",
    digest_file = "
    use blake2::Digest;

    let mut file = std::fs::File::open(path.as_ref())?;
    let mut hasher = Blake2b512::new();
    let digested = std::io::copy(&mut file, &mut hasher)?;
    let hash = Self::from(hasher.finalize());
",
);

impl From<&GenericArray<u8, U32>> for Blake2s256String {
    fn from(value: &GenericArray<u8, U32>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<GenericArray<u8, U32>> for Blake2s256String {
    fn from(value: GenericArray<u8, U32>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<&GenericArray<u8, U32>> for Blake2s256StringUpper {
    fn from(value: &GenericArray<u8, U32>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<GenericArray<u8, U32>> for Blake2s256StringUpper {
    fn from(value: GenericArray<u8, U32>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<&GenericArray<u8, U64>> for Blake2b512String {
    fn from(value: &GenericArray<u8, U64>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<GenericArray<u8, U64>> for Blake2b512String {
    fn from(value: GenericArray<u8, U64>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<&GenericArray<u8, U64>> for Blake2b512StringUpper {
    fn from(value: &GenericArray<u8, U64>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}

impl From<GenericArray<u8, U64>> for Blake2b512StringUpper {
    fn from(value: GenericArray<u8, U64>) -> Self {
        Self::encode_bytes(value).unwrap()
    }
}
