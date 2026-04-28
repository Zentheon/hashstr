use digest::consts::U24;
use hash_str_derive::impl_hash_str;
use tiger::{Tiger, Tiger2};

impl_hash_str!(hasher = tiger::Tiger, con = U24);

impl_hash_str!(hasher = tiger::Tiger2, con = U24);
