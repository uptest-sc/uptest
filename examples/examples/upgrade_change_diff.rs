/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

/// get a diff of storage items that have changed during a runtime upgrade
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::{event_watch, get_raw_metadata, get_runtime_version};
use tokio::time::{sleep, Duration};

use libuptest::pallet_storage_parse::{parse_pallet_storage_types, storage_map_info};

#[tokio::main]
async fn main() -> anyhow::Result<(), libuptest::error::Error> {
    let client = JsonrpseeClient::with_default_url().unwrap();//.expect("Could not connect to chain");
    let old_version = get_runtime_version(client.clone()).await.unwrap();
    println!(
        "Connected to: {:?}  Runtime version: {:?}",
        old_version.spec_name, old_version.spec_version
    );
    let runtime_upgrade_event: event_summary = event_summary {
        pallet_name: "Sudo".to_string(),
        pallet_method: "sudo_unchecked_weight".to_string(),
    };
    let block_limit: u32 = 100u32;
    println!("Waiting for custom event to be triggered");
    let old_metadatablob = get_raw_metadata(client.clone()).await?;
    let old_pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(old_metadatablob).await.unwrap();
    let event_grab: Result<H256, libuptest::error::Error> =
        event_watch(client.clone(), runtime_upgrade_event, block_limit).await;
    println!("Event detected in block: {:?}", event_grab.unwrap());
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

    // Print the changed items
    for item in &changed_items {
        //let old_query: storage_map_info =  *item.to_owned();
        let old: &storage_map_info = old_pallet_list.iter().find(|elem| elem.storage_item_name == item.storage_item_name).expect("Could not detect the previous storage item type");
        //old = old_pallet_list.get_mut::<storage_map_info>(old_query.into());
        println!(
            "Pallet name:  {:?}
Storage item name:  {:?} 
Storage item type:  {:?} 
Old storage type:  {:?}
New storage type: {:?}
", item.pallet_name, item.storage_item_name, item.storage_type, old.raw_type, item.raw_type
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

    println!("All good");
    Ok(())
}
