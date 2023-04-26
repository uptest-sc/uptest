
//use libuptest::jsonrpseeclient::rpcstuff::ExampleHash;
use libuptest::jsonrpseeclient::JsonrpseeClient;
//use libuptest::ws_mod::{get_latest_finalized_head, get_metadata_version};
//use libuptest::jsonrpseeclient::subscription::Request;
use libuptest::types::H256;
//use libuptest::jsonrpseeclient::rpcstuff::RpcParams;
//use jsonrpsee::ws_client::{WsClientBuilder};
//use libuptest::ws_mod::WsClient;
//use jsonrpsee::rpc_params;
//use jsonrpsee_core::client::ClientT;
//use jsonrpsee_core::client::Client;

use libuptest::ws_mod::{get_latest_finalized_head};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
  //  let dial_edg: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();//.unwrap();//.unwrap();
  //  let local_connection: JsonrpseeClient = JsonrpseeClient::with_default_url().unwrap();
 
    println!("ping");

    let client = JsonrpseeClient::edgeware_default_url().unwrap();

    let finalized_block_hash: H256 = get_latest_finalized_head(client).await.unwrap();
    println!("The latest finalized head is: {:?}", finalized_block_hash);
    Ok(())
}

