use digest::{consts::U20, typenum::Unsigned};
use fstr::FStr;
use hash_strings_derive::{StringWrapper, impl_hash_string_tests};
use sha1_checked::{
    Sha1,
    digest::{DynDigest, generic_array::GenericArray},
};

macro_rules! impl_sha1_checked  {
    ($ident:ident)  => {
        impl From<sha1_checked::CollisionResult> for CollisionResult<$ident> {
            fn from(res: sha1_checked::CollisionResult) -> Self {
                let hash_string = res.hash().into();
                match res {
                    sha1_checked::CollisionResult::Ok(_) => CollisionResult::Ok(hash_string),
                    sha1_checked::CollisionResult::Mitigated(_) => {
                        CollisionResult::Mitigated(hash_string)
                    }
                    sha1_checked::CollisionResult::Collision(_) => {
                        CollisionResult::Collision(hash_string)
                    }
                }
            }
        }
        impl From<&GenericArray<u8, U20>> for $ident {
            fn from(value: &GenericArray<u8, U20>) -> Self {
                Self::encode_bytes(value).unwrap()
            }
        }
        impl From<GenericArray<u8, U20>> for $ident {
            fn from(value: GenericArray<u8, U20>) -> Self {
                Self::encode_bytes(value).unwrap()
            }
        }
        impl $ident {
            fn finalize(hasher: Sha1) -> CollisionResult<Self> {
                let res = hasher.try_finalize();
                res.into()
            }
            pub fn dingus() {}
            /// Used for tests. [`Self::try_digest`] is the public API.
            pub fn digest(data: impl AsRef<[u8]>) -> Self {
                let mut hasher = Sha1::default();
                hasher.update(data.as_ref());

                let result = Self::finalize(hasher);

                #[cfg(feature = "tracing")]
                tracing::trace!(?result, "Generated a checked sha1 of raw data");

                result.take_hash()
            }

            /// Attempt to digest the given content into a collision-check sha1 hash.
            pub fn try_digest(data: impl AsRef<[u8]>) -> CollisionResult<Self> {
                let mut hasher = Sha1::default();
                hasher.update(data.as_ref());
                let result = Self::finalize(hasher);

                #[cfg(feature = "tracing")]
                tracing::trace!(?result, "Generated a checked sha1 of raw data");

                result
            }

            pub fn digest_reader<R>(
                read: &mut R,
            ) -> Result<CollisionResult<Self>, std::io::Error>
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
            ) -> Result<CollisionResult<Self>, std::io::Error> {
                let mut file = std::fs::File::open(path.as_ref())?;
                let mut hasher = Sha1::new();
                let digested = std::io::copy(&mut file, &mut hasher)?;

                let result = Self::finalize(hasher);

                #[cfg(feature = "tracing")]
                tracing::trace!(digested, ?result, path = ?path.as_ref(), "Generated a checked sha1 of a file");

                Ok(result)
            }
        }
    }
}

/// Reimplementation of the [`sha1_checked::CollisionResult`] for [`Sha1CheckedString`]
///
/// This enum implements `From<sha1_checked::CollisionResult>`
#[derive(Debug)]
pub enum CollisionResult<T> {
    /// No collision.
    Ok(T),
    /// Collision occured, but was mititgated.
    Mitigated(T),
    /// Collision occured, the hash is the one that collided.
    Collision(T),
}

impl<T> CollisionResult<T> {
    /// Returns the string-wrapped output hash.
    pub fn hash(&self) -> &T {
        match self {
            CollisionResult::Ok(hash) => hash,
            CollisionResult::Mitigated(hash) => hash,
            CollisionResult::Collision(hash) => hash,
        }
    }

    /// Consumes result and returns the string-wrapped output hash.
    pub fn take_hash(self) -> T {
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

#[derive(Debug, Clone, Eq, StringWrapper)]
#[hash_strings(hash_name = "Sha1Checked", con = U20)]
pub struct Sha1CheckedString(FStr<{ U20::USIZE * 2 }>);

impl_sha1_checked!(Sha1CheckedString);

#[derive(Debug, Clone, Eq, StringWrapper)]
#[hash_strings(hash_name = "Sha1Checked", con = U20, upper)]
pub struct Sha1CheckedStringUpper(FStr<{ U20::USIZE * 2 }>);

impl_sha1_checked!(Sha1CheckedStringUpper);

impl_hash_string_tests!(hasher = Sha1, hash_name = "Sha1Checked", con = U20,);
