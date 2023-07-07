/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

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
