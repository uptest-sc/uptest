use libuptest::error::Error;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::pallet_storage_parse::{
    parse_pallet_storage_types, storage_map_info, tmp_metadata, type_id_to_type_def, TypeDef,
    TypeDefComposite, TypeDefReg, TypeDefTuple,
};
use libuptest::test_helper::InputHelper;
use libuptest::ws_mod::get_raw_metadata;

/// convert typeid to raw type, supporting untracked symbols
async fn track_it(typeid: u32, client: JsonrpseeClient) -> Result<(), Error> {
    let raw_metadata: Vec<u8> = get_raw_metadata(client).await?;
    let metadata_scale: &[u8] = &raw_metadata;
    let metadata: tmp_metadata = tmp_metadata::from_bytes(metadata_scale).expect("valid metadata");
    let storage_types = metadata.types.clone();
    println!("Trying to find typeid: {typeid:?}");
    let og_types = storage_types.types();
    for g in og_types.iter() {
        if g.id() == typeid {
            println!("Found it!");
            println!("Type definition: {:?}", g.ty().type_def());
        }
    }
    Ok(())
}

/// detect pallet function input types and generate tests for em
#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    let client = JsonrpseeClient::with_default_url()?;

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
        // "let item_type_input: <TYPE> = 123u32; "
        let random_input: String = match &pallet_info.raw_type {
            TypeDef::Primitive(value) => {
                match value {
                    TypeDefReg::U128 => {
                        //              println!("Found u128");
                        let inp = InputHelper::get_u128();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u128 = {}u128", inp)
                    }
                    TypeDefReg::U64 => {
                        //                println!("Found u64");
                        let inp = InputHelper::get_u64();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);

                        format!("let testinput: u64 = {}u64", inp)
                    }
                    TypeDefReg::U32 => {
                        //                  println!("Found u32");
                        let inp = InputHelper::get_u32();
                        let valname = &pallet.storage_item_name;

                        let outputt = format!("fn submit_to_{}(testinput)", valname);
                        println!("{}", outputt);
                        format!("let testinput: u32 = {}u32", inp)
                    }
                    TypeDefReg::U8 => {
                        //                   println!("Found u8");
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
                    .unwrap() // need fixing upstream
                    .type_name()
                    .ok_or(Error::StorageItemNotFound)?; // only does first, todo: fix this to an iterating solution
                let outputt = format!(
                    "let output: {} = query_storage_map({})",
                    name_type, &pallet.storage_item_name
                );
                //   println!("TypeDefComposite detected with typeid: {:?}", name_type);
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
                //    println!("raw_typeid: {:?}", raw_type);
                //    println!("raw_type_decode: {:?}", type_decode);
                //    let oute = track_it(raw_type, client.clone()).await;

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
                //   let typeid: u32 = value.type_param().id();
                //    println!("Type id is: {:?}", typeid);
                // not really working with this type
                //      let type_decode = type_id_to_type_def(metadatablob.clone(), typeid).await?;
                //      println!("Type type_decode is: {:?}", type_decode);
                //    println!("found sequence!!");
                let outputt = format!(
                    "let Query_chain_state = {}.{}(); // query the {:?} | Sequence storage type",
                    &pallet.pallet_name, &pallet.storage_item_name, &pallet.storage_type
                );

                outputt
            }
            //TypeDef::Seq
            _ => "not detected".to_string(),
        };

        println!("random input is: {:?}", random_input);
        println!("-------------------");
    }

    Ok(())
}
