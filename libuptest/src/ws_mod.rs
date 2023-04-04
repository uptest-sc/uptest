use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::{WsClientBuilder, WsClient};
use crate::jsonrpseeclient::rpcstuff::*;
use jsonrpsee_core::client::async_client::Client;
use jsonrpsee_core::rpc_params;
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::jsonrpseeclient::subscription::Request;
//use crate::jsonrpseeclient::JsonrpseeClient;
//struct wsconnection{
//    connection: ClientT,
//}

/*
// return the metadata version from a ws url
pub async fn get_remote_metadata_version(url: &str) -> anyhow::Result<u8> {
    let client = WsClientBuilder::default().build(&url).await?;
	let hex_data: String = client.request("state_getMetadata", rpc_params![].into()).await?; // get metadata hex blob
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4]) // metadata version
}
*/

pub async fn get_metadata_version(client: JsonrpseeClient) -> anyhow::Result<u8> {
    let hex_data: String = client.request("state_getMetadata", RpcParams::new()).unwrap(); // get metadata hex blob
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4]) 
}

/*
/// get block nr and system block events

/// rpc chain.getBlock(hash)

/// system number

///  chain.getBlockHash
*/