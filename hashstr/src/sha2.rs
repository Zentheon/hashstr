use digest::consts::{U28, U32, U48, U64};
use hashstr_derive::impl_hashstr;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

impl_hashstr!(hasher = sha2::Sha224, con = U28);

impl_hashstr!(hasher = sha2::Sha256, con = U32);

impl_hashstr!(hasher = sha2::Sha384, con = U48);

impl_hashstr!(hasher = sha2::Sha512, con = U64);

impl_hashstr!(hasher = sha2::Sha512_224, con = U28);

impl_hashstr!(hasher = sha2::Sha512_256, con = U32);
