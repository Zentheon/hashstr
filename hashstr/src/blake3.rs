use blake3::{Hash, Hasher};
use digest::consts::U32;
use hashstr_derive::{HashStrDigest, HashStrWrapper, impl_hashstr_tests};

#[derive(Clone, Debug, Eq, HashStrWrapper, HashStrDigest)]
#[hashstr(hasher = blake3::Hash, con = U32, hash_name = "Blake3",
    digest = "
        let mut hasher = Hasher::new();
        hasher.update(data.as_ref());
        let hash = Self::from(hasher.finalize());
    ",
    digest_reader = "
        let mut hasher = Hasher::new();
        let digested = std::io::copy(read, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
    digest_file = "
        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Hasher::new();
        let digested = std::io::copy(&mut file, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
)]
pub struct Blake3Str(fstr::FStr<64>);

#[derive(Clone, Debug, Eq, HashStrWrapper, HashStrDigest)]
#[hashstr(hasher = blake3::Hash, con = U32, hash_name = "Blake3", encoding = "upper_hex",
    digest = "
        let mut hasher = Hasher::new();
        hasher.update(data.as_ref());
        let hash = Self::from(hasher.finalize());
    ",
    digest_reader = "
        let mut hasher = Hasher::new();
        let digested = std::io::copy(read, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
    digest_file = "
        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Hasher::new();
        let digested = std::io::copy(&mut file, &mut hasher)?;
        let hash = Self::from(hasher.finalize());
    ",
)]
pub struct Blake3StrUpper(fstr::FStr<64>);

impl From<Hash> for Blake3Str {
    fn from(value: Hash) -> Self {
        Self(fstr::FStr::try_from(value.to_hex().as_bytes()).unwrap())
    }
}
impl From<Hash> for Blake3StrUpper {
    fn from(value: Hash) -> Self {
        Self(crate::convert_hex_case_fstr::<64, true>(
            &fstr::FStr::try_from(value.to_hex().as_bytes()).unwrap(),
        ))
    }
}

impl_hashstr_tests!(hasher = Hash, hash_name = "Blake3", con = U32,);
