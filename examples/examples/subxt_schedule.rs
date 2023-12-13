// waits until event X is triggered then submits a subxt transaction

use libuptest::subxt_helper::tx_schedule;
use libuptest::types::event_summary;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
use libuptest::error::Error;

#[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
pub mod nodetemplate {}

use nodetemplate::runtime_types::sp_weights::weight_v2::Weight;

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    println!("starting");
    // wait for 100 blocks
    let block_limit: u32 = 100u32;
    // send from alice
    let from_account = dev::alice();
    let dest = dev::bob().public_key().into();
    let event: event_summary = event_summary {
        pallet_name: "Balances".to_string(),
        pallet_method: "transfer_keep_alive".to_string(),
    };

    // transaction to submit
    let call = nodetemplate::tx().balances().transfer(dest, 10u128);
    let ws_host = "ws://127.0.0.1:9944";

    // build the subxt api
    let api = OnlineClient::<PolkadotConfig>::from_url(ws_host)
        .await
        ?;
    // run function
    let go = tx_schedule(from_account, api, &call, event, block_limit, ws_host).await;
    println!(
        "[main]Tx sent in block: {:?}",
        go.expect("Could not get block that tx executed in")
    );

    println!("finish");
    Ok(())
}
