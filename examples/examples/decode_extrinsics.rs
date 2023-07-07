/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
use libuptest::decode_extrinsic::decode_extrinsic_hex_string;
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::types::event_summary;
use libuptest::ws_mod::get_raw_metadata;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raw_extrinsic = "0x280403000ba0ada8438801"; // time stamp extrinsic taken from random polkadot block
    println!("Raw extrinsic value: {raw_extrinsic:?}");
    println!("Downloading metadata");
    let metadata: Vec<u8> = get_raw_metadata(JsonrpseeClient::polkadot_default_url().unwrap())
        .await
        .unwrap(); // yolo
    println!("Metadata downloaded ok");
    let decoded_output = decode_extrinsic_hex_string(raw_extrinsic, &metadata);
    let single_event: event_summary = event_summary {
        pallet_name: decoded_output.call_data.pallet_name.to_string(),
        pallet_method: decoded_output.call_data.ty.name().to_string(),
    };
    let string_vec_events: Vec<event_summary> = vec![single_event];
    println!(
        "Decoded output as:  {:?} ",
        string_vec_events[0].pallet_method
    );
    Ok(())
}
