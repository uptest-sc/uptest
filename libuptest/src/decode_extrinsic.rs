#[cfg(feature = "metadatadecode")] // using desub from parity atm to decode extrinsics and manage the encodements
use desub_current::{
	decoder::{self, Extrinsic},
	Metadata,
};

#[cfg(feature = "metadatadecode")]
fn to_bytes(hex_str: &str) -> Vec<u8> {
	let hex_str = hex_str.strip_prefix("0x").expect("0x should prefix hex encoded bytes");
	hex::decode(hex_str).expect("valid bytes from hex")
}
#[cfg(feature = "metadatadecode")]
pub fn decode_extrinsic_hex_string<'a>(hexstring: &str, metadatablob: &[u8]) -> Extrinsic<'a> {
  //  let jsoninstance = JsonrpseeClient
    let V14_METADATA_POLKADOT_SCALE: &[u8] = metadatablob;
    let ext_bytes = &mut &*to_bytes(hexstring);
    let metadata: Metadata = Metadata::from_bytes(V14_METADATA_POLKADOT_SCALE).expect("valid metadata");
    let output: Extrinsic = decoder::decode_extrinsic(&metadata, ext_bytes).expect("can decode extrinsic");
    output.into_owned()
}