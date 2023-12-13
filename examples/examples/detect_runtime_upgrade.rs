/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use libuptest::decode_extrinsic::decode_extrinsic_hex_string;
/// Detect what pallet methods was called, check if the chain has triggered a special function such as
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, PreBlock, H256};
use libuptest::ws_mod::{
    get_block_events, get_decoded_extrinsics_from_blockhash, get_latest_finalized_head,
    get_raw_metadata, get_runtime_version,
};
use libuptest::error::Error;
// There are several ways of detecting a runtime upgrade, subscribing to each block, parsing each block for the runtime upgrade method to be executed or just probing for RuntimeVersion changes

// todo check if migrations has been triggered

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    // different chains can customize the chain events, the default upgraded chain event is system.setCode
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "system".to_string(),
        pallet_method: "setCode".to_string(),
    }; // define the pallet name and method used to exectue to runtime upgrade, normally system codeupdate
    let client: JsonrpseeClient = JsonrpseeClient::with_default_url()?; // change me
    let metadatablob: Vec<u8> = get_raw_metadata(client.clone()).await?;
    println!("Catching latest block");
    let blockhash: H256 = get_latest_finalized_head(client.clone()).await?; // get_latest_finalized_head(client.clone()).await.unwrap();//H256::from_str("0x89a5dde6705d345117f442dfacf02f4a59bf5cea3ab713a5c07fc4cd78be3a31").unwrap();//
    println!("Got block hash: {blockhash:?}");
    println!("Gathering extrinsics inside block");
    let decodedevent_list: Vec<event_summary> =
        get_decoded_extrinsics_from_blockhash(blockhash, metadatablob.clone(), client.clone())
            .await
            ?;
    println!("List created");
    let contains_runtime_upgrade: bool = decodedevent_list.contains(&runtime_upgrade_event);

    let outputen: PreBlock = get_block_events(blockhash, client.clone()).await?;

    for exti in outputen.block.extrinsics {
        let _printme = decode_extrinsic_hex_string(&exti, &metadatablob);
        //    println!("Decoded arguments: {:?}", printme.call_data.ty);//.call_data.ty);
    }

    match contains_runtime_upgrade {
        true => println!("Contains runtime upgrade!"),
        _ => println!("does not contain runtime upgrade"),
    }
    println!("Getting current runtime version");
    let runtimeversion = get_runtime_version(client).await?;
    println!("Current runtime version: {:?}", runtimeversion.spec_version);
    Ok(())
}
