/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
/// return information about the storage maps and storage values for all enabled pallets
///
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::pallet_storage_parse::{parse_pallet_storage_types, storage_map_info};
use libuptest::ws_mod::get_raw_metadata;
use libuptest::error::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting");
    let client = JsonrpseeClient::with_default_url()?;

    // get the chain's metadata
    let metadatablob = get_raw_metadata(client).await?;

    let pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(metadatablob).await?;
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
    Ok(())
}
