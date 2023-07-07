/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use crate::types::storage_types;
/// parse the storage values and storage maps associated with all the pallets
use desub_current::scale_info::TypeDefPrimitive;

use desub_current::{
    scale_info::form::{Form, PortableForm},
    Metadata,
};

type TypeDef = desub_current::scale_info::TypeDef<PortableForm>;
use frame_metadata::v14::StorageEntryType; // v14 only rn..

#[derive(Debug, PartialEq)]
pub struct storage_map_info {
    pub pallet_name: String,
    pub storage_item_name: String, // name of storagemap
    pub type_id: u32,              // take the type id and query the type_id to type function

    pub raw_type: desub_current::scale_info::TypeDef<PortableForm>,
    pub storage_type: storage_types,
}

struct MyTypeDef;

impl Default for MyTypeDef {
    fn default() -> Self {
        MyTypeDef // Return your custom default value for MyTypeDef
    }
}

impl Form for MyTypeDef {
    type Type = PortableForm;
    type String = String;
    // Implement the required methods from the Form trait
    // ...
}

type TypeDef2 = desub_current::scale_info::TypeDef<MyTypeDef>;

impl storage_map_info {
    /// return a default storage_map_info, so we can easily make it mutable and fill with data from the chain
    /// storage_map_info { pallet_name: "not set", storage_item_name: "not set", type_id: 0 }
    fn new_empty() -> storage_map_info {
        let default_value = create_default_type_def();

        storage_map_info {
            pallet_name: "not set".to_string(),
            storage_item_name: "not set".to_string(),
            type_id: 0,
            storage_type: storage_types::Unknown,
            raw_type: default_value,
        }
    }
}

/// Return a custom default value for TypeDef<MyTypeDef>
fn create_default_type_def() -> desub_current::scale_info::TypeDef<PortableForm> {
    TypeDef::Primitive(TypeDefPrimitive::Bool)
}

/// parses the storagemaps and storage values from all pallets   
/// let raw_metadata = get_raw_metadata(client)   
/// function can not yet detect type id key of storage value so it defaults to 0
/// let pallets_storage_info = parse_pallet_storage_types(raw_metadata).await?;
#[maybe_async::maybe_async(?Send)]
pub async fn parse_pallet_storage_types(
    raw_metadata: Vec<u8>,
) -> anyhow::Result<Vec<storage_map_info>> {
    let metadata_polkadot_scale: &[u8] = &raw_metadata;
    let metadata: Metadata = Metadata::from_bytes(metadata_polkadot_scale).expect("valid metadata");
    let storage_types = metadata.types.clone();
    let og_types = storage_types.types();
    //  println!("got storage types.. : {:?}", og_types);

    let storage_entries = metadata.get_storage_entries();
    //println!("Single: {:?}", storage_entries);

    let mut pallet_list: Vec<storage_map_info> = vec![];
    for item in storage_entries {
        //   println!("Pallet name: {:?}", &item.prefix());
        let current_pallet_name = item.prefix();

        //println!("Storage entries:");

        // detect if type id's has changed

        for entry_to_scan in item.entries() {
            let mut pallet_info: storage_map_info = storage_map_info::new_empty();
            pallet_info.pallet_name = current_pallet_name.clone().to_string();
            pallet_info.storage_item_name = entry_to_scan.name.clone();
            //    println!("Raw entry: {:?}", entry_to_scan);
            let storage_ent = &entry_to_scan.ty.to_owned();
            // println!("Storage item: {:?}", storage_ent);
            //println!("Storage item name: {:?}", entry_to_scan.name);
            match storage_ent {
                StorageEntryType::Plain(_) => {
                    //                    println!("Storage value detected");
                    pallet_info.storage_type = storage_types::StorageValue;
                }
                // we only need the type def right now, lets ignore the hashing type and key atm
                StorageEntryType::Map {
                    hashers: _,
                    key: _,
                    value,
                } => {
                    let typid: u32 = value.id();
                    pallet_info.type_id = typid.clone();
                    //                println!("StorageMap detected with type id key: {:?}", typid);
                    // make better
                    for co in og_types.iter() {
                        //    println!("detecting type");
                        //       pallet_info.por_type = co.clone();
                        if co.id() == typid {
                            pallet_info.raw_type = co.ty().type_def().clone();
                            //                          println!("Type detected: {:?}", co.ty().type_def());
                        }
                    }
                    pallet_info.storage_type = storage_types::StorageMap;
                }
                _ => {
                    //                println!("Could not detect storage type");
                    pallet_info.storage_type = storage_types::Unknown;
                }
            }
            pallet_list.push(pallet_info);
        }
        //     println!("---------------------------------------\r\n");
    }
    // println!("bytes are: ");
    //println!("Amount of pallets:  {:?}", pallet_list.len());
    Ok(pallet_list)
}
