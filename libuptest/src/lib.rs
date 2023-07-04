#![feature(error_in_core)]

mod chains; // depricate me
pub mod codec;
mod connect;
pub mod error;
pub mod jsonrpseeclient;
pub mod metadata;
pub mod types;
pub mod ws_mod;

#[cfg(feature = "subxthelper")]
pub mod subxt_helper;

#[cfg(feature = "metadatadecode")]
pub mod decode_extrinsic;
#[cfg(feature = "metadatadecode")]
pub mod pallet_storage_parse;

/*
pub struct PalletTest {
    pallet_name: String,
    pallet_method: String,

}

impl PalletTest {
    fn new() -> PalletTest {
        PalletTest {
            pallet_name: "test".to_string(),
            pallet_method: "test".to_string()
        }
    }
}

pub fn test() -> PalletTest  {
    PalletTest::new()
}

*/
