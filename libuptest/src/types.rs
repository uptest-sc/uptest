use fixed_hash::construct_fixed_hash;
//use std::str::FromStr;
use serde::{Serialize, Deserialize};


construct_fixed_hash! {
    /// My 256 bit hash type.
    #[derive(Serialize, Deserialize)]
    pub struct H256(32);
}


