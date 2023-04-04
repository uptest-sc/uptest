mod chains;
//mod edgeware;
mod connect;
pub mod ws_mod;
mod error;
pub mod jsonrpseeclient;


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