use libuptest::decode_extrinsic::decode_extrinsic_hex_string;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::ws_mod::get_raw_metadata;
use libuptest::types::event_summary;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raw_extrinsic = "0x280403000ba0ada8438801"; // time stamp extrinsic taken from random polkadot block
    println!("Raw extrinsic value: {raw_extrinsic:?}");
    println!("Downloading metadata");
    let metadata: Vec<u8> = get_raw_metadata(JsonrpseeClient::polkadot_default_url().unwrap()).await.unwrap(); // yolo
    println!("Metadata downloaded ok");
    let decoded_output = decode_extrinsic_hex_string(raw_extrinsic, &metadata);
    let single_event: event_summary = event_summary { pallet_name:  decoded_output.call_data.pallet_name.to_string(), pallet_method: decoded_output.call_data.ty.name().to_string()};
    let string_vec_events: Vec<event_summary> = vec![single_event]; 
    println!("Decoded output as:  {:?} ", string_vec_events[0].pallet_method);
    Ok(())
}