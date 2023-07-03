use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::event_watch;

// the event we want to find

async fn pre_upgrade() -> bool {
    // submit something to pallet X client.send

    true
}

async fn post_upgrade() -> bool {
    true
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Created event");
    let find_me: event_summary = event_summary {
        pallet_method: "Sudo".to_string(),
        pallet_name: "fluff".to_string(),
    };
    let client = JsonrpseeClient::with_default_url().unwrap(); // connect to ws://127.0.0.1:9944
    let block_limit = 200; // subscribe and check the latest 200 blocks for the events
    println!("Looking for event in the {block_limit:?} latest blocks");
    let found_blockhash: H256 = event_watch(client, find_me, block_limit).await.unwrap(); // find our event

    // get the diff log, compare storage maps from metadata

    println!("found event in block: {found_blockhash:?}");
    Ok(())
}
