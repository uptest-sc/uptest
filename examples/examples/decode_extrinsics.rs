use libuptest::decode_extrinsic::decode_extrinsic_hex_string;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::ws_mod::get_raw_metadata;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raw_extrinsic = "0x280403000ba0ada8438801"; // time stamp extrinsic taken from random polkadot block
    println!("Raw extrinsic value: {raw_extrinsic:?}");
    println!("Downloading metadata");
    let metadata: Vec<u8> = get_raw_metadata(JsonrpseeClient::polkadot_default_url().unwrap()).await.unwrap(); // yolo
    println!("Metadata downloaded ok");
    let decoded_output = decode_extrinsic_hex_string(raw_extrinsic, &metadata);
    println!("Decoded output as:  {decoded_output:?}");
    Ok(())
}