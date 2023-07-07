/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

#[cfg(feature = "metadatadecode")]
// using desub from parity atm to decode extrinsics and manage the encodements
use desub_current::{
    decoder::{self, Extrinsic},
    Metadata,
};
//use frame_metadata::v14::StorageEntryType;

use crate::types::{event_summary, pallet_storage_types};

/// take a string, decode it as a Vec<> of u8
#[cfg(feature = "metadatadecode")]
fn to_bytes(hex_str: &str) -> Vec<u8> {
    let hex_str = hex_str
        .strip_prefix("0x")
        .expect("0x should prefix hex encoded bytes");
    hex::decode(hex_str).expect("valid bytes from hex")
}

/// decode a raw extrinsic hex string, take the chains metadata as input
#[cfg(feature = "metadatadecode")]
pub fn decode_extrinsic_hex_string<'a>(hexstring: &str, metadatablob: &[u8]) -> Extrinsic<'a> {
    let metadata_polkadot_scale: &[u8] = metadatablob;
    let ext_bytes = &mut &*to_bytes(hexstring);
    let metadata: Metadata = Metadata::from_bytes(metadata_polkadot_scale).expect("valid metadata");
    let output: Extrinsic =
        decoder::decode_extrinsic(&metadata, ext_bytes).expect("can decode extrinsic");
    output.into_owned()
}
#[cfg(feature = "metadatadecode")]
pub fn decodec_to_event_summary<'a>(extrins: Extrinsic) -> event_summary {
    let single_event: event_summary = event_summary {
        pallet_name: extrins.call_data.pallet_name.to_string(),
        pallet_method: extrins.call_data.ty.name().to_string(),
    };
    //  let string_vec_events: Vec<event_summary> = vec![single_event];
    single_event
}

/// return a Vec of Pallets and the storage items associated with each pallets then use it for diffs
#[cfg(feature = "metadatadecode")]
pub fn pallet_info() -> Vec<pallet_storage_types> {
    todo!("wip")
}
