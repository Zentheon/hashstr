use crate::{EncodingError, Error};
use blake3::{Hash, Hasher};
use digest::consts::U16;
use hash_strings_derive::StringWrapper;

#[derive(Clone, Debug, Eq, StringWrapper)]
#[hash_strings(hasher = Hash, con = U16, hash_name = "Blake3", no_io_wrapper)]
pub struct Blake3String(pub fstr::FStr<32>);

impl Default for Blake3String {
    fn default() -> Self {
        Self::digest([])
    }
}
impl From<Hash> for Blake3String {
    fn from(value: Hash) -> Self {
        Self(fstr::FStr::try_from(value.as_bytes()).unwrap())
    }
}

impl Blake3String {
    /// Digest some raw data and produce a hash.
    pub fn digest(data: impl AsRef<[u8]>) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(data.as_ref());
        let hash = Self::from(hasher.finalize());

        #[cfg(feature = "tracing")]
        tracing::trace!(%hash, "Generated a Blake3 hash of raw data");

        hash
    }
    /// Attempts to digest the entirety of the given reader.
    ///
    /// Returns errors produced by [`std::io::copy`]
    pub fn digest_reader<R>(read: &mut R) -> Result<Self, std::io::Error>
    where
        R: Sized,
        R: std::io::Read,
    {
        let mut hasher = Hasher::new();
        let digested = std::io::copy(read, &mut hasher)?;
        let hash = Self::from(hasher.finalize());

        #[cfg(feature = "tracing")]
        tracing::trace!(digested, %hash, "Generated a Blake3 hash of content in a reader");

        Ok(hash)
    }
    /// Attempts to open the given path and digest the entirety of its bytes.
    ///
    /// Returns errors produced by [`std::fs::File::open`] and [`std::io::copy`]
    pub fn digest_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Hasher::new();
        let digested = std::io::copy(&mut file, &mut hasher)?;
        let hash = Self::from(hasher.finalize());

        #[cfg(feature = "tracing")]
        tracing::trace!(digested, %hash, path = ?path.as_ref(), "Generated a Blake3 hash of a file");

        Ok(hash)
    }
}
