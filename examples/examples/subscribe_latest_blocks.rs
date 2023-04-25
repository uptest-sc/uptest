
use libuptest::jsonrpseeclient::rpcstuff::ExampleHash;
use libuptest::jsonrpseeclient::JsonrpseeClient;
//use libuptest::ws_mod::{get_latest_finalized_head, get_metadata_version};
use libuptest::jsonrpseeclient::subscription::Request;
use libuptest::types::H256;
use libuptest::jsonrpseeclient::rpcstuff::RpcParams;
use jsonrpsee::ws_client::{WsClientBuilder};
use libuptest::ws_mod::WsClient;
use jsonrpsee::rpc_params;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::client::Client;

use libuptest::ws_mod::get_latest_finalized_head;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dial_edg: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();//.unwrap();//.unwrap();
    let local_connection: JsonrpseeClient = JsonrpseeClient::with_default_url().unwrap();
 //   let edg_version: u8 = get_metadata_version(dial_edg.clone()).await?;
//  let finalize_head: ExampleHash = client.request("chain_", RpcParams::new()).unwrap();
    println!("ping");

    let client = JsonrpseeClient::with_default_url().unwrap();
    let url = "ws://127.0.0.1:9944";
//	let clientL  = WsClientBuilder::default().build(&url).await.unwrap();
//    let finalized_block_hash: String =  clientL.request("chain_getFinalizedHead", rpc_params![]).await?;
//
 //   let testout = client.request(method, params)

    let testme: H256 = get_latest_finalized_head(client).await?;
//    let client: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();
//   let dahash: H256 = local_connection.request("chain_getFinalizedHead", RpcParams::new()).unwrap(); // array to H256
 //   println!("The latest finalized head is: {:?}", finalized_block_hash);
    Ok(())
}