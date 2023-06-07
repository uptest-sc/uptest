use libuptest::decode_extrinsic::decode_extrinsic_hex_string;
/// Detect what pallet methods was called, check if the chain has triggered a special function such as
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, PreBlock, H256};
use libuptest::ws_mod::{
    get_block_events, get_decoded_extrinsics_from_blockhash, get_latest_finalized_head,
    get_raw_metadata, get_runtime_version,
};
use std::str::FromStr;

// There are several ways of detecting a runtime upgrade, subscribing to each block, parsing each block for the runtime upgrade method to be executed or just probing for RuntimeVersion changes

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // different chains can customize the chain events, the default upgraded chain event is system.setCode
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "system".to_string(),
        pallet_method: "setCode".to_string(),
    }; // define the pallet name and method used to exectue to runtime upgrade, normally system codeupdate
    let client: JsonrpseeClient = JsonrpseeClient::with_default_url().unwrap(); // change me
    let metadatablob: Vec<u8> = get_raw_metadata(client.clone()).await.unwrap();
    println!("Catching latest block");
    let blockhash: H256 =
        H256::from_str("0x89a5dde6705d345117f442dfacf02f4a59bf5cea3ab713a5c07fc4cd78be3a31")
            .unwrap(); //get_latest_finalized_head(client.clone()).await.unwrap();
    println!("Got block hash: {blockhash:?}");
    println!("Gathering extrinsics inside block");
    let decodedevent_list: Vec<event_summary> =
        get_decoded_extrinsics_from_blockhash(blockhash, metadatablob.clone(), client.clone())
            .await
            .unwrap();
    println!("List created");
    let contains_runtime_upgrade: bool = decodedevent_list.contains(&runtime_upgrade_event);

    let outputen: PreBlock = get_block_events(blockhash, client.clone()).await.unwrap();

    for exti in outputen.block.extrinsics {
        let printme = decode_extrinsic_hex_string(&exti, &metadatablob);
        println!("Decoded arguments: {:?}", printme.call_data.ty);
    }

    match contains_runtime_upgrade {
        true => println!("Contains runtime upgrade!"),
        _ => println!("does not contain runtime upgrade"),
    }
    println!("Getting current runtime version");
    let runtimeversion = get_runtime_version(client).await.unwrap();
    println!("Current runtime version: {:?}", runtimeversion);
    Ok(())
}
