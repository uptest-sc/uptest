// Uptest --chain X --wshost ws://host:port --pallet-test scheduler --sudo "seed goes here"

use clap::ArgMatches;
use libuptest::{jsonrpseeclient::JsonrpseeClient, pallet_storage_parse::storage_map_info};
use std::ffi::OsStr;
mod cli;
mod helper;

#[tokio::main]
async fn main() {
    println!("Uptest command line tool");
    let matches: ArgMatches = cli::gen_cli().get_matches();
    println!("Matches: {:?}", matches.subcommand_name());

    match matches.subcommand_name() {
        Some("pallet-method-sub") => {
            let sub_m = matches.subcommand_matches("pallet-method-sub").unwrap();
            let pallet_method = sub_m.get_one::<String>("pallet_method").map(|s| s.as_str());
            let pallet_name = sub_m.get_one::<String>("pallet_name").map(|s| s.as_str());
            //      println!("Pallet name: {pallet_name:?} Pallet method: {pallet_method:?}");
            //      println!("pallet method sub ok");
            helper::watch_for_event(
                "ws://127.0.0.1:9944",
                pallet_name.unwrap(),
                pallet_method.unwrap(),
            )
            .await;
        }
        Some("storage-changes") => {
            let matched = matches.subcommand_matches("storage-changes").unwrap();
            let ws_host = matched.get_one::<String>("ws_host").unwrap();
            // Use the `ws_host` value in your client code
            println!("ws_host: {}", ws_host);
        }

        Some("pallets-storage") => {
            let sub_m = matches.subcommand_matches("pallets-storage").unwrap();
            let ws_host: String = sub_m.get_one::<String>("ws").unwrap().to_owned();

            // some input validation
            match &ws_host[0..5] == "ws://" {
                true => {}
                false => {
                    panic!("ws host does not start with ws://, double check ws address");
                }
            }

            //            let wshost: &str = sub_m.get_one::<&str>("ws_host").map(|s| s).unwrap();
            println!("Gathering information about all pallet's storage information");
            let listan: Vec<storage_map_info> =
                helper::get_all_pallets_storage(ws_host.as_str()).await;
            for mypallet in listan.iter() {
                println!("Pallet name: {:?}\r\n - Storage item name: {:?}\r\n - Storage type: {:?}\r\n - Storage type id key: {:?}\r\n - Pallet Raw type: {:?}", 
                &mypallet.pallet_name, &mypallet.storage_item_name, &mypallet.storage_type, &mypallet.type_id, &mypallet.raw_type
            );
            }
        }
        Some("pallet-storage") => {
            let sub_m = matches.subcommand_matches("pallet-storage").unwrap();
            let ws_host: String = sub_m.get_one::<String>("ws").unwrap().to_owned();
            let pallet_name: String = sub_m.get_one::<String>("pallet_name").unwrap().to_owned();

            println!("Gathering information about pallet: {pallet_name:?}");
            let listan: Vec<storage_map_info> =
                helper::get_single_pallet_storage(&ws_host, &pallet_name).await;
            match listan.len() {
                0 => {
                    println!("Could not find any pallet by that name, check spelling");
                }
                _ => {
                    println!("Jackpot!");
                    for pallets in listan.iter() {
                        println!("Storage item name: {:?} \r\nType id: {:?} \r\nStorage Type: {:?} \r\n Raw function type: {:?}",
                        pallets.storage_item_name, pallets.type_id, pallets.storage_type, pallets.raw_type
                    );
                    }
                }
            }
        }

        Some("block-watch") => {
            let sub_m = matches.subcommand_matches("block-watch").unwrap();
            let wshost: String = sub_m.get_one::<String>("ws").unwrap().to_owned();
            let mut dalimit: u32 = 0;
            if let Some(c) = sub_m.get_one::<String>("block_limit") {
                let k:  u32 = c.parse::<u32>().unwrap();;
                dalimit = k;
                println!("Value for blocklimit: {c}");
            }
            
            //let block_amount: &u32 = sub_m.get_one("blocklimit").unwrap(); 
          //  let block_amount: u32 = sub_m.get_one::<u32>("blocklimit").unwrap().to_owned();
           // let block_amount: u32 = sub_m.get_one::<u32>("block_limit").unwrap().to_owned();

            let _runner = helper::event_summary_for_latest_blocks(&wshost, dalimit).await;
            println!("all good");
        }
        Some("submit-wasm") => {

            /*
            let sub_m = matches.subcommand_matches("submit-wasm").unwrap();
            let file_path: &OsStr = sub_m.get_one::<&OsStr>("wasm_filepath").map(|s| s).unwrap();
            let tmp_client = JsonrpseeClient::with_default_url().unwrap();
            helper::submit_wasm_runtime_upgrade(tmp_client, &file_path).await;
            */
            todo!("todo, impl signers");
        }
        _ => println!("invalid arguments"),
    }

    /*
    //   println!("Get one: {:?}", matches.subcommand_matches("pallet-method-sub").unwrap());
       if let Some(sub_m) = matches.subcommand_matches("pallet-method-sub") {
           let pallet_method = sub_m.get_one::<String>("pallet_method").map(|s| s.as_str());
           let pallet_name = sub_m.get_one::<String>("pallet_name").map(|s| s.as_str());
           println!("Pallet name: {pallet_name:?}      method: {pallet_method:?}");
       }

    println!(
        "Raw match {:?}",
        matches
            .subcommand()
            .to_owned()
            .unwrap()
            .1
            .get_raw("pallet_name")
    );


    */
}
