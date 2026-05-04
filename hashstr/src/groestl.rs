use digest::consts::{U28, U32, U48, U64};
use groestl::{Groestl224, Groestl256, Groestl384, Groestl512};
use hashstr_derive::impl_hashstr;

impl_hashstr!(hasher = groestl::Groestl224, con = U28);

impl_hashstr!(hasher = groestl::Groestl256, con = U32);

impl_hashstr!(hasher = groestl::Groestl384, con = U48);

impl_hashstr!(hasher = groestl::Groestl512, con = U64);
