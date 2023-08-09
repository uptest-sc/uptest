/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

#[cfg(feature = "subxthelper")]
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::Keypair;
//use subxt::signer::PairSigner;
//use subxt::PairSigner;

use crate::error::Error;
use crate::types::{event_summary, H256};
//use crate::ws_mod::event_watch;
use crate::jsonrpseeclient::JsonrpseeClient;

#[cfg(feature = "metadatadecode")]
use crate::ws_mod::event_watch;

/// schedule a subxt transaction to happen before or after a user defined event is triggered
//use subxt;
pub async fn tx_schedule<Call: subxt::tx::TxPayload>(
    from_account: Keypair,
    api: OnlineClient<PolkadotConfig>,
    call: &Call,
    event: event_summary,
    block_limit: u32,
    ws_host: &str,
) -> Result<H256, Error> {
    //    println!("Waiting for event to trigger");
    //    println!("event set: {:?}", event);
    let client = JsonrpseeClient::new(ws_host).expect("could not create jsonrpsee client");
    let _wait_for_event = event_watch(client, event, block_limit).await;
    // let signer = PairSigner::<PolkadotConfig>::new(from_account);

    //    println!("Event triggered in: {:?}", wait_for_event);
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(call, &from_account)
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();
    //    println!("tx sent");
    let blockhash: H256 = events.block_hash().into();
    // return the block hash that includes the executed transaction
    Ok(blockhash)
}
