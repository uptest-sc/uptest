/// fork a chains state so we can clone the data stored on the chain and use it in a local instance
use libuptest::error::Error;
use libuptest::jsonrpseeclient::JsonrpseeClient;
/// modify a raw chainspec and replace the sudo key with alice key
fn insert_alice_sudo() {}

/// download the chainspec of a chain
async fn get_chainspec(client: JsonrpseeClient) -> bool {
    true
}

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    let client = JsonrpseeClient::with_default_url()?;
    // let raw_chain_spec =

    Ok(())
}
