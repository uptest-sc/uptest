/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
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
