use libuptest::error::Error;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::pallet_storage_parse::{
    parse_pallet_storage_types, storage_map_info, TypeDef, TypeDefTuple, TypeDefComposite, TypeDefReg,
};
use libuptest::test_helper::InputHelper;
use libuptest::ws_mod::get_raw_metadata;
use std::any::Any;

async fn current_pallet_functions(pallet_name: String) -> Result<(), Error> {
    Ok(())
}

/// detect pallet function input types and generate tests for em
#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    let client = JsonrpseeClient::with_default_url()?;

    let metadatablob = get_raw_metadata(client).await?;
    //get a list of the pallets metadata that we can parse throw
    let pallet_list: Vec<storage_map_info> = parse_pallet_storage_types(metadatablob).await?;
    println!("Connect to chain");
    println!("Scanning storage...");

    for pallet in pallet_list.iter() {
        let pallet_info: &storage_map_info = pallet;
        println!("Pallet: {:?}", pallet_info.pallet_name);
        println!("Raw Type: {:?}", pallet_info.raw_type);
            println!("Storage Item name {:?}", pallet.storage_item_name);
        println!("Storage Item type {:?}", pallet.storage_type);
        /// "let item_type_input: <TYPE> = 123u32; "
        let random_input: String = match &pallet_info.raw_type {
            TypeDef::Primitive(value) => {
                match value {
                    TypeDefReg::U128 => {
                        println!("Found u128");
                        let inp = InputHelper::get_u128();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u128 = {}u128", inp)
                    }
                    TypeDefReg::U64 => {
                        println!("Found u64");
                        let inp = InputHelper::get_u64();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);

                        format!("let testinput: u64 = {}u64", inp)
                    }
                    TypeDefReg::U32 => {
                        println!("Found u32");
                        let inp = InputHelper::get_u32();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u32 = {}u32", inp)
                    }
                    TypeDefReg::U8 => {
                        println!("Found u8");
                        let inp = InputHelper::get_u8();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u8 = {}u8", inp)
                    }
                    TypeDefReg::Bool => {
                        // accountid
                        let outputt = format!("fn submit_to_{}(testinput)", &pallet.storage_item_name);
                        println!("{}", outputt);
                        let testinput: bool = InputHelper::get_boolean();
                        format!("let testinput: bool = {}", testinput)
                    }
              
                    _ => "not detected".to_string(),
                }
            }
          
            TypeDef::Composite(value) => {
                let test = value; //
                let name_type = test.fields().first().unwrap().type_name().ok_or(Error::StorageItemNotFound)?; // only does first, todo: fix this to an iterating solution
                let outputt = format!("let output: {} = query_storage_map({})", name_type, &pallet.storage_item_name);
             //   println!("TypeDefComposite detected with typeid: {:?}", name_type);
                outputt
            },

            TypeDef::Tuple(value) => {
                let outputt = format!("query_storage_map({})", &pallet.storage_item_name);

                println!("tuple detected");
                outputt.to_string()
            }
           
            _ => "not detected".to_string(),
        };

        println!("random input is: {:?}", random_input);
        println!("-------------------");
    }

    Ok(())
}
