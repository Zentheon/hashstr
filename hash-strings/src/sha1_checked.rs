use crate::{EncodingError, Error};
use digest::{consts::U20, typenum::Unsigned};
use fstr::FStr;
use hash_strings_derive::{StringWrapper, impl_hash_string_tests};
use sha1_checked::{
    Sha1,
    digest::{DynDigest, generic_array::GenericArray},
};

#[derive(Debug, Clone, Eq, StringWrapper)]
#[hash_strings(hash_name = "Sha1Checked", con = U20)]
pub struct Sha1CheckedString(FStr<{ U20::USIZE * 2 }>);

/// Reimplementation of the [`sha1_checked::CollisionResult`] for [`Sha1CheckedString`]
///
/// This enum implements `From<sha1_checked::CollisionResult>`
#[derive(Debug)]
pub enum CollisionResult {
    /// No collision.
    Ok(Sha1CheckedString),
    /// Collision occured, but was mititgated.
    Mitigated(Sha1CheckedString),
    /// Collision occured, the hash is the one that collided.
    Collision(Sha1CheckedString),
}

impl CollisionResult {
    /// Returns the string-wrapped output hash.
    pub fn hash(&self) -> &Sha1CheckedString {
        match self {
            CollisionResult::Ok(hash) => hash,
            CollisionResult::Mitigated(hash) => hash,
            CollisionResult::Collision(hash) => hash,
        }
    }

    /// Consumes result and returns the string-wrapped output hash.
    pub fn take_hash(self) -> Sha1CheckedString {
        match self {
            CollisionResult::Ok(hash) => hash,
            CollisionResult::Mitigated(hash) => hash,
            CollisionResult::Collision(hash) => hash,
        }
    }

    /// Returns if there was a collision
    pub fn has_collision(&self) -> bool {
        !matches!(self, CollisionResult::Ok(_))
    }
}

impl From<sha1_checked::CollisionResult> for CollisionResult {
    fn from(res: sha1_checked::CollisionResult) -> Self {
        let hash_string = res.hash().into();
        match res {
            sha1_checked::CollisionResult::Ok(_) => CollisionResult::Ok(hash_string),
            sha1_checked::CollisionResult::Mitigated(_) => CollisionResult::Mitigated(hash_string),
            sha1_checked::CollisionResult::Collision(_) => CollisionResult::Collision(hash_string),
        }
    }
}

impl From<&GenericArray<u8, U20>> for Sha1CheckedString {
    fn from(digest: &GenericArray<u8, U20>) -> Self {
        Self(base16ct::lower::encode_string(digest).try_into().unwrap())
    }
}

impl From<GenericArray<u8, U20>> for Sha1CheckedString {
    fn from(digest: GenericArray<u8, U20>) -> Self {
        Self::from(&digest)
    }
}

impl Sha1CheckedString {
    fn finalize(hasher: Sha1) -> CollisionResult {
        let res = hasher.try_finalize();
        res.into()
    }
    /// Attempt to digest the given content into a collision-check sha1 hash.
    pub fn try_digest(data: impl AsRef<[u8]>) -> CollisionResult {
        let mut hasher = Sha1::default();
        hasher.update(data.as_ref());

        let result = Self::finalize(hasher);

        #[cfg(feature = "tracing")]
        tracing::trace!(?result, "Generated a checked sha1 of raw data");

        result
    }

    pub fn digest_reader<R>(read: &mut R) -> Result<CollisionResult, std::io::Error>
    where
        R: Sized,
        R: std::io::Read,
    {
        let mut hasher = Sha1::new();
        let digested = std::io::copy(read, &mut hasher)?;

        let result = Self::finalize(hasher);

        #[cfg(feature = "tracing")]
        tracing::trace!(
            digested,
            ?result,
            "Generated a checked sha1 of content in a reader"
        );

        Ok(result)
    }

    pub fn digest_file(
        path: impl AsRef<std::path::Path>,
    ) -> Result<CollisionResult, std::io::Error> {
        let mut file = std::fs::File::open(path.as_ref())?;
        let mut hasher = Sha1::new();
        let digested = std::io::copy(&mut file, &mut hasher)?;

        let result = Self::finalize(hasher);

        #[cfg(feature = "tracing")]
        tracing::trace!(digested, ?result, path = ?path.as_ref(), "Generated a checked sha1 of a file");

        Ok(result)
    }
}

impl_hash_string_tests!(
    hasher = Sha1,
    hash_name = "Sha1Checked",
    con = U20,
    ignore_tests = ["digest"]
);
