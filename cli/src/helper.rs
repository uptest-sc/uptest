/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

This is a uptest-cli helper module to utilize libuptest

*/

use libuptest::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use libuptest::jsonrpseeclient::subscription::HandleSubscription;
use libuptest::jsonrpseeclient::subscription::Subscribe;
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams, SubscriptionWrapper};
use libuptest::pallet_storage_parse::{parse_pallet_storage_types, storage_map_info};
use libuptest::types::Header;
use libuptest::types::{event_summary, RuntimeVersion, H256};
use libuptest::ws_mod::{
    blocknumber_to_blockhash, event_watch, get_block_events, get_raw_metadata, get_runtime_version,
};
use tokio::time::sleep;
use tokio::time::Duration;

use libuptest::error::Error;
use libuptest::test_generation::autogen::generate_test_std;
// generate_test_std

pub async fn auto_test(wshost: &str) -> Result<(), Error> {
    let client = JsonrpseeClient::new(wshost)?;
    let _out = generate_test_std(client).await?;
    Ok(())
}

/// display meta information about chain X
pub async fn chain_info(wshost: &str) -> bool {
    let client = JsonrpseeClient::new(wshost).unwrap();
    let chain_info: RuntimeVersion = get_runtime_version(client).await.unwrap();
    println!("----Chain-Info----");
    println!(
        "Chain Name: {:?}
Runtime version: {:?}
Authoring Version: {:?}
State Version: {:?}",
        chain_info.spec_name,
        chain_info.spec_version,
        chain_info.authoring_version,
        chain_info.state_version
    );
    println!("--E-O-L--");
    true
}

/// return all storagevalues and storagemaps for all pallets
pub async fn get_all_pallets_storage(wshost: &str) -> Vec<storage_map_info> {
    let client = JsonrpseeClient::new(wshost).unwrap();
    // get the chain's metadata
    let metadatablob = get_raw_metadata(client).await.unwrap();

    let pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(metadatablob).await.unwrap();
    pallet_list
}

/// subscribe to a chain, wait for runtime upgrade to be triggered, display changes in the storage item types before and after
pub async fn storage_changes(
    ws: &str,
    block_limit: u32,
) -> anyhow::Result<(), libuptest::error::Error> {
    //Vec<pallet_storage_diff> {
    let client = JsonrpseeClient::new(ws).expect("Could not connect to ws endpoint");
    println!("Connected to chain");
    let old_version = get_runtime_version(client.clone())
        .await
        .expect("could not get RuntimeVersion from chain");
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "Sudo".to_string(),
        pallet_method: "sudo_unchecked_weight".to_string(),
    };
    let old_metadatablob = get_raw_metadata(client.clone()).await?;
    let old_pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(old_metadatablob).await.unwrap();
    println!("Gathered current storage types");
    println!("Waiting for runtime upgrade");
    let event_grab: H256 = event_watch(client.clone(), runtime_upgrade_event, block_limit)
        .await
        .expect("could not get runtime upgrade block");
    println!("Runtime upgrade in block: {:?}", event_grab);
    println!("Having a coffee break before next block...");
    let duration_to_wait = Duration::new(10, 0); // chill 10 seconds
    let _ = sleep(duration_to_wait).await;
    // diff the predata and the new data
    println!("Scanning the new metadata for changes");
    let new_metadatablob = get_raw_metadata(client.clone()).await?;
    let new_pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(new_metadatablob).await.unwrap();
    let new_version = get_runtime_version(client.clone()).await.unwrap();
    println!(
        "Runtime upgraded from version: {:?} to new version: {:?}",
        old_version.spec_version, new_version.spec_version
    );

    // check which items only the type has been changed, storagemap could have changed type but not name
    let changed_items: Vec<_> = new_pallet_list
        .iter()
        .filter(|new_item| {
            old_pallet_list
                .iter()
                .find(|old_item| {
                    old_item.pallet_name == new_item.pallet_name
                    && old_item.storage_item_name == new_item.storage_item_name
                  //  && old_item.type_id == new_item.type_id
                    && old_item.raw_type != new_item.raw_type
                })
                .is_some()
        })
        .collect();
    if changed_items.len() == 0 {
        println!("No storage items has been changed");
    }
    // Print the changed items
    for item in &changed_items {
        //let old_query: storage_map_info =  *item.to_owned();
        let old: &storage_map_info = old_pallet_list
            .iter()
            .find(|elem| elem.storage_item_name == item.storage_item_name)
            .expect("Could not detect the previous storage item type");
        //old = old_pallet_list.get_mut::<storage_map_info>(old_query.into());
        println!(
            "Pallet name:  {:?}
    Storage item name:  {:?} 
    Storage item type:  {:?} 
    Old storage type:  {:?}
    New storage type: {:?}
    ",
            item.pallet_name,
            item.storage_item_name,
            item.storage_type,
            old.raw_type,
            item.raw_type
        );
    }
    // lets check the new storage items that has been added to the runtime
    let added_items: Vec<_> = new_pallet_list
        .iter()
        .filter(|new_item| {
            old_pallet_list
                .iter()
                .find(|old_item| {
                    old_item.pallet_name == new_item.pallet_name
                        && old_item.storage_item_name == new_item.storage_item_name
                })
                .is_none()
        })
        .collect();

    for sitem in added_items.iter() {
        println!(
            "Pallet: {:?} has added a {:?} with the type: {:?}",
            sitem.pallet_name, sitem.storage_type, sitem.raw_type
        );
    }

    Ok(())
}

/// return all storagevalues and storagemaps for one single pallets
pub async fn get_single_pallet_storage(wshost: &str, pallet_name: &str) -> Vec<storage_map_info> {
    let pallet_list: Vec<storage_map_info> = get_all_pallets_storage(wshost).await;
    let new_list: Vec<storage_map_info> = pallet_list
        .into_iter()
        .filter(|pallet_entry: &storage_map_info| pallet_entry.pallet_name == pallet_name)
        .collect(); // filter list based on pallet name
    new_list
}

/// display what pallet and functions where triggers in the X amount of latest finalized blocks
pub async fn event_summary_for_latest_blocks(wshost: &str, block_amount: u32) -> bool {
    let client = JsonrpseeClient::new(wshost).unwrap(); // change me
    let metadatablob = get_raw_metadata(client.clone()).await.unwrap();
    println!("Subscribing to latest finalized blocks at {wshost:?}");
    let mut subscrib: SubscriptionWrapper<Header> = client
        .clone()
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        .unwrap();

    for _ in 0..block_amount {
        let tmp_client = JsonrpseeClient::new(wshost).unwrap();
        let nextone = subscrib.next();
        let blocknr = nextone.unwrap().unwrap().number;
        println!("------------------------------------------------");
        println!("Latest finalized block number: #{}", blocknr);
        let blockhash: H256 = blocknumber_to_blockhash(tmp_client.clone(), blocknr.clone())
            .await
            .unwrap();
        println!("Finalized block hash: {blockhash:?}");

        let preblock = get_block_events(blockhash, tmp_client).await.unwrap();

        let extrinsics = preblock.block.extrinsics;

        let decodedevent_list: Vec<event_summary> = extrinsics
            .clone()
            .iter()
            .map(|n| {
                decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob))
            })
            .collect();

        for eventet in decodedevent_list.into_iter() {
            println!(
                "[Triggered event] Pallet: {} triggered event: {}",
                eventet.pallet_name, eventet.pallet_name
            );
        }
        println!("------------------------------------------------\r\n");
    }

    let _ = subscrib.unsubscribe();
    true //Ok(true)
}

/// Subscribe and break on user defined event
pub async fn watch_for_event(wshost: &str, pallet_name: &str, pallet_method: &str) -> bool {
    println!("Subscribing to Chain X, Metadata Version Y");
    println!("Connecting to chain..");
    let client = JsonrpseeClient::new(wshost).unwrap();
    let findme_event: event_summary = event_summary {
        pallet_name: pallet_name.to_string(),
        pallet_method: pallet_method.to_string(),
    };
    println!("Looking for Pallet: {:?}", findme_event.pallet_name);
    let mut subscrib: SubscriptionWrapper<Header> = client
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        .unwrap();
    let metadatablob = get_raw_metadata(client.clone()).await.unwrap();

    for _ in 0..100 {
        let nextone = subscrib.next();
        let blocknr = nextone.unwrap().unwrap().number;
        let blockhash: H256 = blocknumber_to_blockhash(client.clone(), blocknr.clone())
            .await
            .unwrap(); // can we change this syntax so we are not looping clone's?

        println!("Checking block #{}", blocknr);
        let preblock = get_block_events(blockhash, client.clone()).await.unwrap();
        let extrinsics = preblock.block.extrinsics;
        println!("Got block events... Decoding it..");
        let decodedevent_list: Vec<event_summary> = extrinsics
            .clone()
            .iter()
            .map(|n| {
                decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob))
            })
            .collect();
        match decodedevent_list.contains(&findme_event) {
            true => {
                println!("Block: {:?} contains event", blockhash);
                panic!("Exiting..");
            }
            false => println!(
                "Block: {:?} does not contain Pallet: {:?} with method: {:?}",
                blockhash, findme_event.pallet_name, findme_event.pallet_method
            ),
        };
    }

    let _ = subscrib.unsubscribe();
    true
}

/*
ts implementation:
  const txid = await api.tx.sudo
    .sudoUncheckedWeight(
      api.tx.system.setCodeWithoutChecks(`0x${buff.toString('hex')}`), 10000
    )
    .signAndSend(sudoac);

*/

/* moved to seperate repo: https://github.com/uptest-sc/submit-runtime-upgrade
pub async fn submit_wasm_runtime_upgrade(
    client: JsonrpseeClient,
    file_path: &std::ffi::OsStr,
) -> bool {
    let bloben: u8 = read_wasm_binary(Path::new(file_path)).await.unwrap(); // read in the wasm file, validate it?
                                                                            // sign and submit the wasm file
    let hex_blob = hex::encode([bloben]);
    //  let submitted = client.request(method, params)
    true
}
*/
