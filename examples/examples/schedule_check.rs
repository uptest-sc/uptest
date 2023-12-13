/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::event_watch;
use libuptest::error::Error;
// the event we want to find

async fn pre_upgrade() -> bool {
    // submit something to pallet X client.send

    true
}

async fn post_upgrade() -> bool {
    true
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    println!("Created event");
    let find_me: event_summary = event_summary {
        pallet_method: "Sudo".to_string(),
        pallet_name: "fluff".to_string(),
    };
    let client = JsonrpseeClient::with_default_url()?; // connect to ws://127.0.0.1:9944
    let block_limit = 200; // subscribe and check the latest 200 blocks for the events
    println!("Looking for event in the {block_limit:?} latest blocks");
    let found_blockhash: H256 = event_watch(client, find_me, block_limit).await?; // find our event

    // get the diff log, compare storage maps from metadata

    println!("found event in block: {found_blockhash:?}");
    Ok(())
}
