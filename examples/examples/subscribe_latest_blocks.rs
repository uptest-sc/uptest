// subscribe to latest blocks, this is only in dev stage, on the way to moving it a better impl

use libuptest::jsonrpseeclient::subscription::HandleSubscription;
use libuptest::jsonrpseeclient::subscription::Subscribe;
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams, SubscriptionWrapper};
use libuptest::types::Header;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = JsonrpseeClient::edgeware_default_url().unwrap();

    // todo: macro this
    println!("Subscribing");
    let mut subscrib: SubscriptionWrapper<Header> = client
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        .unwrap();

    for _ in 0..3 {
        let nextone = subscrib.next();
        println!(
            "Latest finalized block: {:?}",
            nextone.unwrap().unwrap().number
        );
    }

    println!("unsubscribing");
    let _ = subscrib.unsubscribe();
    Ok(())
}
