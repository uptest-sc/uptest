// automatically generate tests that can be used for extrinsic testing
use crate::error::Error;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::test_helper::InputHelper;
use crate::ws_mod::get_raw_metadata;

use crate::pallet_storage_parse::{
    parse_pallet_storage_types, storage_map_info, type_id_to_type_def, TypeDef, TypeDefReg,
};
use crate::types::storage_types;

use crate::pallet_storage_parse::RawType;

/// Auto test generation summary
pub struct AutoTestSummary {
    /// name of pallet
    pub pallet_name: String,
    /// name of storage item
    pub storage_item_name: String,
    /// StorageValue or StorageMap
    pub storage_type: storage_types,
    /// raw type
    pub raw_type: RawType,
    /// suggest a way to query the chain
    pub input_suggest: String,
}

pub type AutoTests = Vec<AutoTestSummary>;

/// generate a summary of the tests for a chain
pub async fn generate_auto_test(client: JsonrpseeClient) -> Result<AutoTests, Error> {
    let mut collect: Vec<AutoTestSummary> = Vec::new();
    let metadatablob = get_raw_metadata(client.clone()).await?;
    //get a list of the pallets metadata that we can parse throw
    let pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(metadatablob.clone()).await?;

    for pallet in pallet_list.iter() {
        let pallet_info: &storage_map_info = pallet;
        let pallet_name: String = pallet_info.pallet_name.to_owned();
        let raw_type = pallet_info.raw_type.to_owned();
        let storage_item_nam = pallet.storage_item_name.to_owned();
        let storage_type: storage_types = pallet.storage_type;

        let mut inphlp: String = String::new();
        let _input_suggest: String = match &pallet_info.raw_type {
            TypeDef::Primitive(value) => {
                match value {
                    TypeDefReg::U128 => {
                        let inp = InputHelper::get_u128();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        inphlp.push_str(&outputt.as_str());
                        let outme = format!("let testinput: u128 = {}u128", inp);
                        inphlp.push_str(outme.as_str());
                        outme
                    }
                    TypeDefReg::U64 => {
                        let inp = InputHelper::get_u64();
                        let valname = &pallet.storage_item_name;
                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        let addme = format!("let testinput: u64 = {}u64", inp);
                        inphlp.push_str(outputt.as_str());
                        inphlp.push_str(addme.as_str());
                        addme
                    }
                    TypeDefReg::U32 => {
                        let inp = InputHelper::get_u32();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        let s = format!("let testinput: u32 = {}u32", inp);
                        inphlp.push_str(s.as_str());
                        inphlp.push_str(&outputt.as_str());
                        s
                    }
                    TypeDefReg::U8 => {
                        let inp = InputHelper::get_u8();
                        let valname = &pallet.storage_item_name;
                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        let s = format!("let testinput: u8 = {}u8", inp);
                        inphlp.push_str(s.as_str());
                        inphlp.push_str(&outputt.as_str());
                        s
                    }
                    TypeDefReg::Bool => {
                        // accountid
                        let outputt =
                            format!("fn submit_to_{}(testinput)", &pallet.storage_item_name);
                        let testinput: bool = InputHelper::get_boolean();
                        let s = format!("let testinput: bool = {}", testinput);
                        inphlp.push_str(s.as_str());
                        inphlp.push_str(&outputt.as_str());
                        s
                    }

                    _ => "not detected".to_string(),
                }
            }

            TypeDef::Composite(value) => {
                let test = value; //
                let name_type = test
                    .fields()
                    .first()
                    .unwrap() // needs looking over upstream
                    .type_name()
                    .ok_or(Error::StorageItemNotFound)?; // only does first, todo: fix this to an iterating solution
                let outputt = format!(
                    "let output: {} = query_storage_map({})",
                    name_type, &pallet.storage_item_name
                );
                inphlp.push_str(&outputt.as_str());
                outputt
            }

            TypeDef::Tuple(value) => {
                // Add strings to the vector
                let mut catch_types: Vec<String> = Vec::new();

                for field_row in value.fields().iter() {
                    let raw_type: u32 = field_row.id();
                    let type_decode = type_id_to_type_def(metadatablob.clone(), raw_type).await?;
                    let add_me = format!("{type_decode:?}");
                    catch_types.push(add_me);
                }

                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?}, output is a tuple of ",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );
                inphlp.push_str(outputt.as_str());
                // convert typeid to raw type
                outputt.to_string()
            }

            TypeDef::Variant(value) => {
                let mut variantz: Vec<String> = Vec::new();
                for entrypoint in value.variants().iter() {
                    let name = entrypoint.name();
                    variantz.push(name.clone());
                }
                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?} ",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );

                outputt
            }

            TypeDef::Sequence(_value) => {
                // wip
                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?} | Sequence storage type",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );
                outputt
            }
            _ => "not detected, type not supported by uptest".to_string(),
        };

        let sum: AutoTestSummary = AutoTestSummary {
            pallet_name: pallet_name,
            storage_item_name: storage_item_nam,
            storage_type: storage_type,
            raw_type: raw_type,
            input_suggest: inphlp,
        };
        collect.push(sum);
    }

    Ok(collect)
}

/// automatically generate tests for a chains storage entries
/// returns output to stdout with a standard println
pub async fn generate_test_std(client: JsonrpseeClient) -> Result<(), Error> {
    let metadatablob = get_raw_metadata(client.clone()).await?;
    //get a list of the pallets metadata that we can parse throw
    let pallet_list: Vec<storage_map_info> =
        parse_pallet_storage_types(metadatablob.clone()).await?;
    println!("Connect to chain");
    println!("Scanning storage...");

    for pallet in pallet_list.iter() {
        let pallet_info: &storage_map_info = pallet;
        println!("Pallet: {:?}", pallet_info.pallet_name);
        println!("Raw Type: {:?}", pallet_info.raw_type);
        println!("Storage Item name {:?}", pallet.storage_item_name);
        println!("Storage Item type {:?}", pallet.storage_type);
        println!("Code:");

        let random_input: String = match &pallet_info.raw_type {
            TypeDef::Primitive(value) => {
                match value {
                    TypeDefReg::U128 => {
                        let inp = InputHelper::get_u128();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u128 = {}u128", inp)
                    }
                    TypeDefReg::U64 => {
                        let inp = InputHelper::get_u64();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);

                        format!("let testinput: u64 = {}u64", inp)
                    }
                    TypeDefReg::U32 => {
                        let inp = InputHelper::get_u32();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u32 = {}u32", inp)
                    }
                    TypeDefReg::U8 => {
                        let inp = InputHelper::get_u8();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u8 = {}u8", inp)
                    }
                    TypeDefReg::Bool => {
                        // accountid
                        let outputt =
                            format!("fn submit_to_{}(testinput)", &pallet.storage_item_name);
                        println!("{}", outputt);
                        let testinput: bool = InputHelper::get_boolean();
                        format!("let testinput: bool = {}", testinput)
                    }

                    _ => "not detected".to_string(),
                }
            }

            TypeDef::Composite(value) => {
                let test = value; //
                let name_type = test
                    .fields()
                    .first()
                    .unwrap() // needs looking over upstream
                    .type_name()
                    .ok_or(Error::StorageItemNotFound)?; // only does first, todo: fix this to an iterating solution
                let outputt = format!(
                    "let output: {} = query_storage_map({})",
                    name_type, &pallet.storage_item_name
                );
                outputt
            }

            TypeDef::Tuple(value) => {
                // Add strings to the vector
                let mut catch_types: Vec<String> = Vec::new();

                for field_row in value.fields().iter() {
                    let raw_type: u32 = field_row.id();
                    let type_decode = type_id_to_type_def(metadatablob.clone(), raw_type).await?;
                    let add_me = format!("{type_decode:?}");
                    catch_types.push(add_me);
                }
                println!("Type definition of tuple output: ");
                println!("tuple (");
                for item in catch_types.iter() {
                    println!("{item:}");
                }
                println!(") // end of tuple");

                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?}, output is a tuple of ",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );
                // convert typeid to raw type
                outputt.to_string()
            }

            TypeDef::Variant(value) => {
                let mut variantz: Vec<String> = Vec::new();
                for entrypoint in value.variants().iter() {
                    let name = entrypoint.name();
                    variantz.push(name.clone());
                }
                println!("Output could be any of the following:");
                for var in variantz {
                    println!("{:?}", var);
                }
                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?} ",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );

                outputt
            }

            TypeDef::Sequence(value) => {
                // wip
                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?} | Sequence storage type",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );
                outputt
            }
            _ => "not detected, type not supported by uptest".to_string(),
        };

        println!("random input is: {:?}", random_input);
        println!("-------------------");
    }

    // raw type to match against the input gen
    Ok(())
}
