/// return information about the storage maps and storage values for all enabled pallets
/// 
/// 


use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::ws_mod::get_raw_metadata;
use libuptest::pallet_storage_parse::{storage_map_info, parse_pallet_storage_types};


#[tokio::main]
async fn main() {
    println!("Starting");
    let client = JsonrpseeClient::with_default_url().unwrap();

    // get the chain's metadata
    let metadatablob = get_raw_metadata(client).await.unwrap();

    let pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(metadatablob).await.unwrap();
    println!("Amount of pallets:  {:?}", pallet_list.len());
    println!("looping throw loot");
    // /*
    for mypallet in pallet_list.iter() {
        println!("Pallet name: {:?}\r\n - Storage item name: {:?}\r\n - Storage type: {:?}\r\n - Storage type id key: {:?}\r\n - Pallet Raw type: {:?}", 
        &mypallet.pallet_name, &mypallet.storage_item_name, &mypallet.storage_type, &mypallet.type_id, &mypallet.raw_type
    );
    }

    //  */
    println!("done");
    //Ok(())
}
