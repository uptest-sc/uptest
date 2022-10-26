/// Connect to chain over ws

use jsonrpsee::{
    async_client::{ClientBuilder, Client},
    client_transport::ws::{
        Uri,
        WsTransportClientBuilder,
    },
    core::{
        client::ClientT,
        Error,
    },
    rpc_params,
};

use std::time::Duration;


// ws client

pub async fn get_ws_client(url: &Uri) -> Result<Client, Error> {
    let (sender, receiver) = WsTransportClientBuilder::default()
    .build(url.to_string().parse::<Uri>().unwrap())
    .await
    .map_err(|e| Error::Transport(e.into()))?;

Ok(ClientBuilder::default()
    .request_timeout(Duration::from_secs(180))
    .max_notifs_per_subscription(4096)
    .build_with_tokio(sender, receiver))
}


/// Extra good to use functions


/// fetch_metadata from a ws host by sending state_getMedata
pub async fn fetch_metadata_ws(url: &Uri) -> Result<String, Error> {
    let client = get_ws_client(url).await?;

    Ok(client.request("state_getMetadata", rpc_params![]).await?)
}
