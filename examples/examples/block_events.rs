use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{PreBlock, H256};
use libuptest::ws_mod::get_block_events;

use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("function started");
    let polkadot_dial = JsonrpseeClient::polkadot_default_url().unwrap();
    println!("Connection established");
    let mablock: H256 =
        H256::from_str("0x8784cba4254c3800f502b0732b0260d0dee3b85701e8cbbd45bdddb7d3d2d5bf")?;
    println!("mablock ok");
    let output: PreBlock = get_block_events(mablock, polkadot_dial).await.unwrap();
    println!("got output: {:?}", output);
    println!("Block Nr: {:?}", output.block.header.number);
    println!("function passed, output:");
    Ok(())
}
