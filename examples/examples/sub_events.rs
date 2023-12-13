/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

/// subscribe to latest blocks, print all events triggered in the latest blocks
use libuptest::jsonrpseeclient::subscription::HandleSubscription;
use libuptest::jsonrpseeclient::subscription::Subscribe;
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams, SubscriptionWrapper};
use libuptest::types::Header;
use libuptest::error::Error;
use libuptest::decode_extrinsic::{decode_extrinsic_hex_string, decodec_to_event_summary};
use libuptest::types::{event_summary, H256};
use libuptest::ws_mod::{blocknumber_to_blockhash, get_block_events, get_raw_metadata};

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    let wsinstance = "wss://polkadot-rpc-tn.dwellir.com:443"; // my node endpoint
    let client: JsonrpseeClient = JsonrpseeClient::new(wsinstance)?;

    println!("Subscribing");
    let mut subscrib: SubscriptionWrapper<Header> = client
        .subscribe::<Header>(
            "chain_subscribeFinalizedHeads",
            RpcParams::new(),
            "chain_unsubscribeFinalizedHeads",
        )
        ?;
    let metadatablob = get_raw_metadata(client.clone()).await?;

    for _ in 0..300 {
        let nextone = subscrib.next();
        println!("------------------------------------------------");
        let myblocknr = match nextone {
            Some(Ok(header)) => header.clone().number,
            Some(Err(_err)) => {
                return Err(Error::BlockparseError);
            }
            None => {
                return Err(Error::BlockparseError);
            }
        };
        println!("Latest finalized block number: #{}", myblocknr);
        let tmp_client: JsonrpseeClient = JsonrpseeClient::new(wsinstance)?;
        let blockhash: H256 = blocknumber_to_blockhash(tmp_client.clone(), myblocknr.clone())
            .await
            ?;
        println!("Block hash: {blockhash:?}");

        //    let tmpblock: H256 = H256::from_str("0x17ee6d42553cf5161144ab95fecfe27c694e697f2d7e6f22271972cf476176b5").unwrap(); static polkadot block used for debugging

        //let tmpblock: H256 = H256::from_string("0x343f3f94ff17c79f2f4e77dcb5e894507b89dd575dbc2e36bde658ad653ead45"); //0x343f3f94ff17c79f2f4e77dcb5e894507b89dd575dbc2e36bde658ad653ead45
        let preblock = get_block_events(blockhash, tmp_client).await?;

        let extrinsics = preblock.block.extrinsics;

        let decodedevent_list: Vec<event_summary> = extrinsics
            .clone()
            .iter()
            .map(|n| {
                decodec_to_event_summary(decode_extrinsic_hex_string(n.as_str(), &metadatablob))
            })
            .collect();
        for eventet in decodedevent_list.into_iter() {
            println!(
                "[Triggered event] Pallet: {} triggered event: {}",
                eventet.pallet_name, eventet.pallet_name
            );
        }
        println!("------------------------------------------------\r\n");
    }

    println!("unsubscribing");
    let _ = subscrib.unsubscribe();
    Ok(())
}
