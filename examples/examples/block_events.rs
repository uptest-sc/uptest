use libuptest::ws_mod::get_block_events;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{H256, PreBlock};


use std::str::FromStr;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("function started");
    let polkadot_dial = JsonrpseeClient::polkadot_default_url().unwrap();
    println!("Connection established");
    let mablock: H256 = H256::from_str("0x453604c1547d86e467d544b5e931e2ed09d966f19c3783923042ae453394cdcd")?;
    println!("mablock ok");
    let _output: PreBlock = get_block_events(mablock, polkadot_dial).await.unwrap();
    println!("got output!");
    println!("function passed, output:");
    Ok(())
}

