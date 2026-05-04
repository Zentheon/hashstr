use digest::consts::U24;
use hashstr_derive::impl_hashstr;
use tiger::{Tiger, Tiger2};

impl_hashstr!(hasher = tiger::Tiger, con = U24);

impl_hashstr!(hasher = tiger::Tiger2, con = U24);
