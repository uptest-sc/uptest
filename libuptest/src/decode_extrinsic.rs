#[cfg(feature = "metadatadecode")]
// using desub from parity atm to decode extrinsics and manage the encodements
use desub_current::{
    decoder::{self, Extrinsic},
    Metadata,
};

use crate::types::event_summary;

#[cfg(feature = "metadatadecode")]
pub fn to_bytes(hex_str: &str) -> Vec<u8> {
    let hex_str = hex_str
        .strip_prefix("0x")
        .expect("0x should prefix hex encoded bytes");
    hex::decode(hex_str).expect("valid bytes from hex")
}

#[cfg(feature = "metadatadecode")]
pub fn decode_extrinsic_hex_string<'a>(hexstring: &str, metadatablob: &[u8]) -> Extrinsic<'a> {
    //  let jsoninstance = JsonrpseeClient
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
