/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

/// for metadata v11 - v14
///
use std::path::Path; // no_std, well....
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::Error;

/*
/// read the runtime upgrade wasm file, non blocking async file read with tokio, this does not work when reading wasm binaries to submit as runtime upgrade
pub async fn read_wasm_binary(file_location: &Path) -> anyhow::Result<u8, Error> {
    let mut file_blob: File = File::open(file_location).await?;
    let mut buffer = [0; 10];
    let n = file_blob.read(&mut buffer).await?;
    Ok(n.try_into().unwrap())
}
*/

/// use this function to convert it to a wasm code you can use to submit a runtime upgrade
/// example usage with subxt:   
///     let wasm_path = Path::new("/tmp/substrate-node-template/target/release/wbuild/node-template-runtime/node_template_runtime.compact.wasm");
///    // read binary
///    let code: Vec<u8> = read_wasm_binary_correct(wasm_path).await;
///    // create system set_code call
///    let call = nodetemplate::runtime_types::node_template_runtime::RuntimeCall::System(
///        SystemCall::set_code {
///            code: code.into(),
///        },
///    ); 
pub async fn read_wasm_binary_correct(file_location: &Path) -> Vec<u8> {
    let filepath = tokio::fs::read(file_location).await.expect("Could not read wasm file");
    filepath
}
