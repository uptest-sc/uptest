/// for metadata v11 - v14
///
use std::path::Path; // no_std, well....
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::Error;

/// read the runtime upgrade wasm file, non blocking async file read with tokio
pub async fn read_wasm_binary(file_location: &Path) -> anyhow::Result<u8, Error> {
    let mut file_blob: File = File::open(file_location).await?;
    let mut buffer = [0; 10];
    let n = file_blob.read(&mut buffer).await?;
    Ok(n.try_into().unwrap())
}
