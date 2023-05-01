pub use jsonrpsee::ws_client::{WsClientBuilder, WsClient};
//use crate::error;
//use crate::jsonrpseeclient::rpcstuff::*;
use crate::rpc_params;
use std::str::FromStr;
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::jsonrpseeclient::subscription::Request;
use crate::types::{H256, PreBlock};


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


/// get the metadata version of chain X
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
pub async fn get_latest_finalized_head(client: JsonrpseeClient) -> anyhow::Result<H256, crate::error::Error> {
    let hex_data: String = client.request("chain_getFinalizedHead", RpcParams::new()).await?;
    let finb: H256 = H256::from_str(&hex_data.as_str())?;
    Ok(finb) 
}




#[maybe_async::maybe_async(?Send)]
pub async fn get_block_events(blockhash: H256, client: JsonrpseeClient) -> anyhow::Result<PreBlock, crate::error::Error> {
    let string_hash: String = format!("{:?}", blockhash);
 //   let query: Vec<String> = client.request("chain_getBlock",  rpc_params!["0x11dc73c97be314034507df6ceb80a3f4e15aa0d04f2a7f148e076e68e5341f96"]).await?; //rpc_params![].into()
   
    let qq: PreBlock = client.request("chain_getBlock",  rpc_params![string_hash]).await?; //rpc_params![].into()
    //let fluff: Vec<String> = vec!["fluff".to_string()];
    Ok(qq)
}

/*
/// get block nr and system block events

/// rpc chain.getBlock(hash)

/// system number

///  chain.getBlockHash
*/