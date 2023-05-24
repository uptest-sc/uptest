use fixed_hash::construct_fixed_hash;
use serde::{Deserialize, Serialize};

construct_fixed_hash! {
    /// My 256 bit hash type.
    #[derive(Serialize, Deserialize)]
    pub struct H256(32);
}

impl H256 {
    fn to_string(&self) -> String {
        let stringen = format!("{:?}", self);
        stringen
    }
}

/// todo add the cargo expand

#[derive(Debug, Deserialize)]
pub struct Header {
    pub number: String,
}

#[derive(Debug, PartialEq)]
pub struct event_summary {
    pub pallet_name: String,
    pub pallet_method: String,
}

/// generic substrate sp-runtime block
#[derive(Debug, Deserialize)]
pub struct Block<Header, Extrinsic: Serialize> {
    pub header: Header,
    pub extrinsics: Extrinsic, //Vec<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct justifications(Vec<u8>);

// what we get if
#[derive(Debug, Deserialize)]
pub struct PreBlock {
    pub block: generic_block,

    pub justifications: Option<justifications>, //Justification can be null so lets put this in an option
}

pub type generic_block = Block<Header, Vec<String>>;
