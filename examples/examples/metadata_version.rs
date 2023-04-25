// get the metadata version of the chain

use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::ws_mod::get_metadata_version;
//use libuptest::jsonrpseeclient::subscription::Request;
//use libuptest::jsonrpseeclient::RpcParams;

//use jsonrpsee::ws_client::{WsClientBuilder, WsClient};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
   /*
    println!("Connecting to polkadot");
    let dial_polkadot = JsonrpseeClient::polkadot_default_url();
    println!("Connecting to Kusama");
    let dial_ksm = JsonrpseeClient::kusama_default_url();
    println!("Connecting to Sora");
    let dial_sora = JsonrpseeClient::sora_default_url();
    */
    
    println!("Connecting to Edgeware");
    let dial_edg: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();//.unwrap();//.unwrap();
    let edg_version: u8 = get_metadata_version(dial_edg).await.unwrap(); // yolo unwrap
   
    println!("Connected to chain: {:?} and got metadata version: {:?}", "Edgeware", edg_version);
    Ok(())
}