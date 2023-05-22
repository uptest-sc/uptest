#![feature(error_in_core)]


mod chains;
//mod edgeware;
mod connect;
pub mod ws_mod;
mod error;
pub mod jsonrpseeclient;
pub mod types;
pub mod codec;

#[cfg(feature = "metadatadecode")]
pub mod decode_extrinsic;

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
