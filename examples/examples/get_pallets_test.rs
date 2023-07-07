/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::H256;
use libuptest::ws_mod::{get_latest_finalized_head, get_raw_metadata};
//use libuptest::jsonrpseeclient::subscription::Request;
//use libuptest::jsonrpseeclient::RpcParams;

//use jsonrpsee::ws_client::{WsClientBuilder, WsClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tmpclient = JsonrpseeClient::with_default_url().unwrap();

    let metadata: Vec<u8> = get_raw_metadata(tmpclient).await.unwrap();
    println!("decode me: ");
    let metadata_scale: &[u8] = metadata;
    let ext_bytes = &mut &*to_bytes(hexstring);
    let metadata: Metadata = Metadata::from_bytes(metadata_polkadot_scale).expect("valid metadata");
    let output: Extrinsic =
        decoder::decode_extrinsic(&metadata, ext_bytes).expect("can decode extrinsic");
    let _ = output.into_owned();

    Ok(())
}
