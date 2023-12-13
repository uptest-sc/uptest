use libuptest::jsonrpseeclient::JsonrpseeClient;
/// https://github.com/uptest-sc/uptest/issues/69
/*
cargo run -p uptest-examples --example chain_info
----Chain-Info----
Chain Name: "node-template"
Runtime version: 109
Authoring Version: 1
State Version: 1
--E-O-L--
*/
use libuptest::types::RuntimeVersion;
use libuptest::ws_mod::get_runtime_version;
use libuptest::error::Error;

#[tokio::main]
async fn main() -> anyhow::Result<(), Error> {
    // chain at 127.0.0.1:9944
    let client = JsonrpseeClient::with_default_url()?;
    let chain_info: RuntimeVersion = get_runtime_version(client).await?;
    println!("----Chain-Info----");
    println!(
        "Chain Name: {:?}
Runtime version: {:?}
Authoring Version: {:?}
State Version: {:?}",
        chain_info.spec_name,
        chain_info.spec_version,
        chain_info.authoring_version,
        chain_info.state_version
    );
    println!("--E-O-L--");
    Ok(())
}
