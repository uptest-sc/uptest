pub use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
//use crate::error;
//use crate::jsonrpseeclient::rpcstuff::*;
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use crate::jsonrpseeclient::subscription::Request;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::rpc_params;
use crate::types::{PreBlock, H256};
use std::str::FromStr;

#[cfg(feature = "metadatadecode")]
use crate::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use crate::types::{event_summary, RuntimeVersion};

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
pub async fn get_metadata_version(
    client: JsonrpseeClient,
) -> anyhow::Result<u8, crate::error::Error> {
    let hex_data: String = client
        .request("state_getMetadata", RpcParams::new())
        .await?;
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4])
}

#[maybe_async::maybe_async(?Send)]
pub async fn get_raw_metadata(
    client: JsonrpseeClient,
) -> anyhow::Result<Vec<u8>, crate::error::Error> {
    let hex_data: String = client
        .request("state_getMetadata", RpcParams::new())
        .await?;
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes)
}

/// Query chain.getBlockHash to get the block hash
#[maybe_async::maybe_async(?Send)]
pub async fn blocknumber_to_blockhash(
    client: JsonrpseeClient,
    block_nr: String,
) -> anyhow::Result<H256, crate::error::Error> {
    let raw_data: String = client
        .request("chain_getBlockHash", rpc_params![block_nr])
        .await?;
    let hashen: H256 = H256::from_str(&raw_data.as_str())?;
    Ok(hashen)
}

/// change to self.client

/// Query chain.getBlock to get the block hash
#[maybe_async::maybe_async(?Send)]
pub async fn blockhash_to_block(
    client: JsonrpseeClient,
    block_hash: H256,
) -> anyhow::Result<H256, crate::error::Error> {
    let raw_data: String = client
        .request("chain_getBlock", rpc_params![block_hash.to_string()])
        .await?;
    let hashen: H256 = H256::from_str(&raw_data.as_str())?;
    Ok(hashen)
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
pub async fn get_latest_finalized_head(
    client: JsonrpseeClient,
) -> anyhow::Result<H256, crate::error::Error> {
    let hex_data: String = client
        .request("chain_getFinalizedHead", RpcParams::new())
        .await?;
    let finb: H256 = H256::from_str(&hex_data.as_str())?;
    Ok(finb)
}

#[maybe_async::maybe_async(?Send)]
pub async fn get_block_events(
    blockhash: H256,
    client: JsonrpseeClient,
) -> anyhow::Result<PreBlock, crate::error::Error> {
    let string_hash: String = format!("{:?}", blockhash);
    //   let query: Vec<String> = client.request("chain_getBlock",  rpc_params!["0x11dc73c97be314034507df6ceb80a3f4e15aa0d04f2a7f148e076e68e5341f96"]).await?; //rpc_params![].into()

    let qq: PreBlock = client
        .request("chain_getBlock", rpc_params![string_hash])
        .await?; //rpc_params![].into()
                 //let fluff: Vec<String> = vec!["fluff".to_string()];
    Ok(qq)
}

#[cfg(feature = "metadatadecode")]
#[maybe_async::maybe_async(?Send)]
pub async fn get_decoded_extrinsics_from_blockhash(
    blockhash: H256,
    metadatablob: Vec<u8>,
    client: JsonrpseeClient,
) -> anyhow::Result<Vec<event_summary>, crate::error::Error> {
    let preblock: PreBlock = get_block_events(blockhash, client).await.unwrap();
    let extrinsics: Vec<String> = preblock.block.extrinsics;
    let decodedevent_list: Vec<event_summary> = extrinsics
        .clone()
        .iter()
        .map(|n| decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob)))
        .collect();

    Ok(decodedevent_list)
}

// get runtime version, state.getRuntimeVersion, different on different chains RuntimeVersion
#[maybe_async::maybe_async(?Send)]
pub async fn get_runtime_version(
    client: JsonrpseeClient,
) -> anyhow::Result<RuntimeVersion, crate::error::Error> {
    let runtimeversion: RuntimeVersion = client
        .request("state_getRuntimeVersion", RpcParams::new())
        .await?;
    Ok(runtimeversion)
}
