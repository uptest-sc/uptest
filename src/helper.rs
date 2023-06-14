use libuptest::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use libuptest::jsonrpseeclient::subscription::HandleSubscription;
use libuptest::jsonrpseeclient::subscription::Subscribe;
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams, SubscriptionWrapper};
use libuptest::metadata::read_wasm_binary;
use libuptest::types::Header;
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::{blocknumber_to_blockhash, get_block_events, get_raw_metadata};
use std::path::Path;

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

        println!("Checking block #{:?}", blocknr);
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

pub async fn submit_wasm_runtime_upgrade(
    client: JsonrpseeClient,
    file_path: &std::ffi::OsStr,
) -> bool {
    let bloben: u8 = read_wasm_binary(Path::new(file_path)).await.unwrap(); // read in the wasm file, validate it?
                                                                            // sign and submit the wasm file
    let hex_blob = hex::encode([bloben]);

    true
}
