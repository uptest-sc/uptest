/// Detect what pallet methods was called, check if the chain has triggered a special function such as
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::{get_latest_finalized_head, get_raw_metadata, get_decoded_extrinsics_from_blockhash};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // different chains can customize the chain events, the default upgraded chain event is system.setCode
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "system".to_string(),
        pallet_method: "setCode".to_string(),
    }; // define the pallet name and method used to exectue to runtime upgrade, normally system codeupdate
    let client: JsonrpseeClient = JsonrpseeClient::polkadot_default_url().unwrap(); // change me
    let metadatablob: Vec<u8> = get_raw_metadata(client.clone()).await.unwrap();
    println!("Catching latest block");
    let blockhash: H256 = get_latest_finalized_head(client.clone()).await.unwrap();
    println!("Got block hash: {blockhash:?}");
    println!("Gathering extrinsics inside block");
    let decodedevent_list: Vec<event_summary> = get_decoded_extrinsics_from_blockhash(blockhash, metadatablob, client.clone()).await.unwrap();
    println!("List created");
    let contains_runtime_upgrade: bool = decodedevent_list.contains(&runtime_upgrade_event);

    match contains_runtime_upgrade {
        true => println!("Contains runtime upgrade!"),
        _ => println!("does not contain runtime upgrade"),
    }

    Ok(())
}
