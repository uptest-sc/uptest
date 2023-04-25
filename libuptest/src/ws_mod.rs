use jsonrpsee::core::client::ClientT;
pub use jsonrpsee::ws_client::{WsClientBuilder, WsClient};
//use crate::error;
//use crate::jsonrpseeclient::rpcstuff::*;
use jsonrpsee_core::client::async_client::Client;
use jsonrpsee_core::rpc_params;
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::jsonrpseeclient::subscription::Request;
use crate::types::H256;

pub struct Wsclientwrapper();

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



#[maybe_async::maybe_async(?Send)]
pub async fn get_metadata_version(client: JsonrpseeClient) -> anyhow::Result<u8, crate::error::Error> {
    let hex_data: String = client.request("state_getMetadata", RpcParams::new()).await?;
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4]) 
}
/*
pub async fn get_metadata_version(client: JsonrpseeClient) -> anyhow::Result<u8> {
    let hex_data: String = client.request("state_getMetadata", RpcParams::new()).unwrap(); // get metadata hex blob
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4]) 
}


// chain_getFinalizedHead
pub async fn get_latest_finalized_head(client: JsonrpseeClient) -> anyhow::Result<H256> {
        let finalize_head: H256 = client.request("chain_getFinalizedHead", RpcParams::new()).unwrap();
        Ok(finalize_head)
}

*/

#[maybe_async::maybe_async(?Send)]
pub async fn get_latest_finalized_head(client: &Client) -> anyhow::Result<H256> {
    let finalize_head: H256 = client.request("chain_getFinalizedHead", rpc_params![]).await?;
    Ok(finalize_head)
}

/*
/// get block nr and system block events

/// rpc chain.getBlock(hash)

/// system number

///  chain.getBlockHash
*/