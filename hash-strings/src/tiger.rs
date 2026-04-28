use digest::consts::U24;
use hash_strings_derive::impl_hash_string;
use tiger::{Tiger, Tiger2};

impl_hash_string!(hasher = tiger::Tiger, con = U24);

impl_hash_string!(hasher = tiger::Tiger2, con = U24);
