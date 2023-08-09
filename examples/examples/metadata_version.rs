/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

// get the metadata version of the chain

use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::H256;
use libuptest::ws_mod::{get_latest_finalized_head, get_metadata_version};
//use libuptest::jsonrpseeclient::subscription::Request;
//use libuptest::jsonrpseeclient::RpcParams;

//use jsonrpsee::ws_client::{WsClientBuilder, WsClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    /*
    println!("Connecting to polkadot");
    let dial_polkadot = JsonrpseeClient::polkadot_default_url();
    println!("Connecting to Kusama");
    let dial_ksm = JsonrpseeClient::kusama_default_url();
    println!("Connecting to Sora");
    let dial_sora = JsonrpseeClient::sora_default_url();
    */

    println!("Connecting to Edgeware");
    let dial_edg: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();
    let edg_version: u8 = get_metadata_version(dial_edg.clone()).await.unwrap(); // yolo unwrap
    println!(
        "Connected to chain: {:?} and got metadata version: {:?}",
        "Edgeware", edg_version
    );
    let finalized_block_hash: H256 = get_latest_finalized_head(dial_edg).await.unwrap();
    //   let str_it = format!("{:?}", finalized_block_hash);
    println!("The latest finalized head is: {:?}", finalized_block_hash);

    Ok(())
}
