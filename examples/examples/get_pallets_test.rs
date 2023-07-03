use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::H256;
use libuptest::ws_mod::{get_latest_finalized_head, get_raw_metadata};
//use libuptest::jsonrpseeclient::subscription::Request;
//use libuptest::jsonrpseeclient::RpcParams;

//use jsonrpsee::ws_client::{WsClientBuilder, WsClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> { 
    let tmpclient = JsonrpseeClient::with_default_url().unwrap();

    let metadata: Vec<u8> = get_raw_metadata(tmpclient).await.unwrap();
    println!("decode me: ");
    let metadata_scale: &[u8] = metadata;
    let ext_bytes = &mut &*to_bytes(hexstring);
    let metadata: Metadata = Metadata::from_bytes(metadata_polkadot_scale).expect("valid metadata");
    let output: Extrinsic =
        decoder::decode_extrinsic(&metadata, ext_bytes).expect("can decode extrinsic");
    output.into_owned()

    Ok(())
 }