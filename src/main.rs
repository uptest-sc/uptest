// Uptest --chain X --wshost ws://host:port --pallet-test scheduler --sudo "seed goes here"

use clap::ArgMatches;
use libuptest::jsonrpseeclient::JsonrpseeClient;
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
        Some("submit-wasm") => {
            let sub_m = matches.subcommand_matches("submit-wasm").unwrap();
            let file_path: &OsStr = sub_m.get_one::<&OsStr>("wasm_filepath").map(|s| s).unwrap();
            let tmp_client = JsonrpseeClient::with_default_url().unwrap();
            helper::submit_wasm_runtime_upgrade(tmp_client, &file_path).await;
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
