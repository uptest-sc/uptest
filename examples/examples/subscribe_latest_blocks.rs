/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
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
