/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

pub use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
//use crate::error;
//use crate::jsonrpseeclient::rpcstuff::*;
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use crate::jsonrpseeclient::subscription::Request;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::types::{PreBlock, H256};
use crate::{metadata, rpc_params};
use std::str::FromStr;

use crate::error::Error;

use crate::error::Error as standard_error;

#[cfg(feature = "metadatadecode")]
use crate::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use crate::jsonrpseeclient::subscription::HandleSubscription;
use crate::jsonrpseeclient::subscription::Subscribe;
use crate::jsonrpseeclient::SubscriptionWrapper;
use crate::types::{event_summary, Header, RuntimeVersion};

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
) -> anyhow::Result<u8, Error> {
    let hex_data: String = client
        .request("state_getMetadata", RpcParams::new())
        .await?;
    let bytes: Vec<u8> = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes[4])
}

/// get the chain's metadata and return it as a Vec<u8>
#[maybe_async::maybe_async(?Send)]
pub async fn get_raw_metadata(client: JsonrpseeClient) -> anyhow::Result<Vec<u8>, standard_error> {
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
) -> anyhow::Result<H256, standard_error> {
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
) -> anyhow::Result<H256, standard_error> {
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

/// get the latest finalized block
#[maybe_async::maybe_async(?Send)]
pub async fn get_latest_finalized_head(
    client: JsonrpseeClient,
) -> anyhow::Result<H256, Error> {
    let hex_data: String = client
        .request("chain_getFinalizedHead", RpcParams::new())
        .await?;
    let finb: H256 = H256::from_str(&hex_data.as_str())?;
    Ok(finb)
}

/// return all block events in event_summary lower case strings
#[cfg(feature = "metadatadecode")]
#[maybe_async::maybe_async(?Send)]
pub async fn get_block_events_lower_case(
    blockhash: H256,
    client: JsonrpseeClient,
    metadata: Vec<u8>,
) -> anyhow::Result<Vec<event_summary>, standard_error> {
    let preblock: PreBlock = get_block_events(blockhash, client).await.unwrap();
    let extrinsics: Vec<String> = preblock.block.extrinsics;
    let decodedevent_list: Vec<event_summary> = extrinsics
        .clone()
        .iter()
        .map(|n| decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadata)))
        .collect();

    // convert to lower case
    let loop_it: Vec<event_summary> = decodedevent_list
        .iter()
        .map(|n| event_summary {
            pallet_name: n.pallet_name.to_ascii_lowercase(),
            pallet_method: n.pallet_method.to_ascii_lowercase(),
        })
        .collect();

    Ok(loop_it)
}

/// get block events in block
#[maybe_async::maybe_async(?Send)]
pub async fn get_block_events(
    blockhash: H256,
    client: JsonrpseeClient,
) -> anyhow::Result<PreBlock, Error> {
    let string_hash: String = format!("{:?}", blockhash);
    //   let query: Vec<String> = client.request("chain_getBlock",  rpc_params!["0x11dc73c97be314034507df6ceb80a3f4e15aa0d04f2a7f148e076e68e5341f96"]).await?; //rpc_params![].into()

    let qq: PreBlock = client
        .request("chain_getBlock", rpc_params![string_hash])
        .await.map_err(|err| Error::CouldNotGetBlock)?; 
    
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

/// get runtime version, state.getRuntimeVersion, different on different chains RuntimeVersion
#[maybe_async::maybe_async(?Send)]
pub async fn get_runtime_version(
    client: JsonrpseeClient,
) -> anyhow::Result<RuntimeVersion, crate::error::Error> {
    let runtimeversion: RuntimeVersion = client
        .request("state_getRuntimeVersion", RpcParams::new())
        .await?;
    Ok(runtimeversion)
}

/// same as event_watch but supports non case sensitive
/// event search, event_summary non case sensitive
pub async fn event_watch_non_case_sensitive(
    client: JsonrpseeClient,
    event_to_find: event_summary,
    block_limit: u32,
) -> anyhow::Result<H256, crate::error::Error> {
    /*
      let fake_blockhash: H256 =
          H256::from_str("0x89a5dde6705d345117f442dfacf02f4a59bf5cea3ab713a5c07fc4cd78be3a31")
              .unwrap();
    */

    // convert the event to lower case in order to match it
    // ascii lowercase match https://doc.rust-lang.org/std/string/struct.String.html#method.to_ascii_lowercase
    let my_event: event_summary = event_summary {
        pallet_name: event_to_find.pallet_name.to_ascii_lowercase(),
        pallet_method: event_to_find.pallet_method.to_ascii_lowercase(),
    };
    //  get_block_events_lower_case get_block_events_lower_case
    let mut subscrib: SubscriptionWrapper<Header> = client
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        .map_err(|err| Error::ConnectionSubscriptionProblem)?;
    let metadatablob: Vec<u8> = get_raw_metadata(client.clone()).await?;

    for _ in 0..block_limit {
        let tmp_client = client.clone(); // change me
        let nextone = subscrib.next();
        let block_nr = nextone.unwrap().unwrap().number; // change me
        let tmp_blockhash = blocknumber_to_blockhash(tmp_client.clone(), block_nr).await?;
        // get block events in event_summary with ascii lowercase String values
        let block_events_lower_case: Vec<event_summary> =
            get_block_events_lower_case(tmp_blockhash, tmp_client.clone(), metadatablob.clone())
                .await?;

        // find the event in the event list
        match block_events_lower_case.contains(&my_event) {
            true => {
                let _ = subscrib.unsubscribe(); // unsubscribe before killing it
                                                // return the hash of the block the event was found in
                return Ok(tmp_blockhash);
            }
            false => continue,
        };
    }

    Err(standard_error::EventNotFound) //Ok(fake_blockhash)
}

/// return the H256 hash of the block the user given event is triggered on
/// client, block event to find, amount of blocks to check
///     let custom_event: event_summary = event_summary {
/// pallet_name: "Sudo".to_string(),
/// pallet_method: "sudo_unchecked_weight".to_string(),
///};
///  let block_limit: u32 = 100u32;
/// let search_n_find = event_watch(client, custom_event, block_limit).await
#[cfg(feature = "metadatadecode")]
#[maybe_async::maybe_async(?Send)]
pub async fn event_watch(
    client: JsonrpseeClient,
    event: event_summary,
    block_limit: u32,
) -> anyhow::Result<H256, crate::error::Error> {
    let metadatablob = get_raw_metadata(client.clone()).await.map_err(|error| Error::NoMetaData)?;
    //  let blockhash: H256 =
    //      H256::from_str("0x89a5dde6705d345117f442dfacf02f4a59bf5cea3ab713a5c07fc4cd78be3a31")
    //          .unwrap();

    let mut subscrib: SubscriptionWrapper<Header> = client
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        .map_err(|error|Error::ConnectionSubscriptionProblem)?;

    for _ in 0..block_limit {
        //        println!("event watch start");
        let tmp_client = client.clone(); // change me
        let nextone = subscrib.next();
        let block_nr = nextone.unwrap().unwrap().number;
        let tmp_blockhash = blocknumber_to_blockhash(tmp_client.clone(), block_nr).await?;
        let preblock = get_block_events(tmp_blockhash, tmp_client).await.map_err(|error| Error::CouldNotGetBlock)?;

        let extrinsics = preblock.block.extrinsics;

        let decodedevent_list: Vec<event_summary> = extrinsics
            .clone()
            .iter()
            .map(|n| {
                decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob))
            })
            .collect();
        //     println!("Events triggered: {:?}", decodedevent_list);
        match decodedevent_list.contains(&event) {
            true => {
                let _ = subscrib.unsubscribe(); // unsubscribe before killing it
                return Ok(tmp_blockhash);
            }
            false => continue,
        };
    }

    println!("unsubscribing");
    let _ = subscrib.unsubscribe();

    Err(standard_error::EventNotFound) // Ok(fake_blockhash) // crate::error::Error::EventNotFound
}
