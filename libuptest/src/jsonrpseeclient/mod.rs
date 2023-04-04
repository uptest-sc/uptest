// based on substrate-api-client 

/*
   Copyright 2019 Supercomputing Systems AG
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
	   http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use crate::error::Error;// {Error, Request, Result, RpcParams, Subscribe};
use futures::executor::block_on;
use jsonrpsee::{
	client_transport::ws::{Uri, WsTransportClientBuilder},
	core::{
		client::{Client, ClientBuilder, ClientT, SubscriptionClientT},
		traits::ToRpcParams,
	},
};
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use std::sync::Arc;

use crate::jsonrpseeclient::subscription::Request;
use crate::jsonrpseeclient::subscription::Subscribe;

pub use subscription::SubscriptionWrapper;
pub use rpcstuff::RpcParams;

pub mod subscription;
pub mod rpcstuff;

pub use subscription::Result;

#[derive(Clone)]
pub struct JsonrpseeClient {
	inner: Arc<Client>,
}

impl JsonrpseeClient {
	pub fn new(url: &str) -> Result<Self> {
		block_on(Self::async_new(url))
	}

	pub fn with_default_url() -> Result<Self> {
		Self::new("ws://127.0.0.1:9944")
	}

	// lets extend with some preconfigured chains
	pub fn edgeware_default_url() -> Result<Self> {
		Self::new("wss://edgeware.jelliedowl.net:443")
	}

	pub fn polkadot_default_url() -> Result<Self> {
		Self::new("wss://polkadot-rpc-tn.dwellir.com:443")
	}

	pub fn kusama_default_url() -> Result<Self> {
		Self::new("wss://kusama-rpc-tn.dwellir.com:443")
	}


	pub fn sora_default_url() -> Result<Self> {
		Self::new("wss://ws.mof.sora.org:443")
	}


	pub fn get_metadata_version() -> Result<u32> {
		Ok(1u32)
	}

	async fn async_new(url: &str) -> Result<Self> {
		let uri: Uri = url.parse().map_err(|e| Error::InvalidUrl(url.to_string()))?;
		let (tx, rx) = WsTransportClientBuilder::default()
			.build(uri)
			.await
			.map_err(|e| Error::ConnectionHandshakefailed)?;
		let client = ClientBuilder::default()
			.max_notifs_per_subscription(4096)
			.build_with_tokio(tx, rx);
		Ok(Self { inner: Arc::new(client) })
	}
}

impl Request for JsonrpseeClient {
	fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
		// Support async: #278
		block_on(self.inner.request(method, RpcParamsWrapper(params)))
			.map_err(|e| Error::ConnectionSubscriptionProblem)
	}
}



impl Subscribe for JsonrpseeClient {
	type Subscription<Notification> = SubscriptionWrapper<Notification> where Notification: DeserializeOwned;

	fn subscribe<Notification: DeserializeOwned>(
		&self,
		sub: &str,
		params: RpcParams,
		unsub: &str,
	) -> Result<Self::Subscription<Notification>> {
		block_on(self.inner.subscribe(sub, RpcParamsWrapper(params), unsub))
			.map(|sub| sub.into())
			.map_err(|e| Error::ConnectionSubscriptionProblem)
	}
}

struct RpcParamsWrapper(crate::jsonrpseeclient::rpcstuff::RpcParams);

impl ToRpcParams for RpcParamsWrapper {
	fn to_rpc_params(self) -> core::result::Result<Option<Box<RawValue>>, jsonrpsee::core::Error> {
		if let Some(json) = self.0.build() {
			RawValue::from_string(json)
				.map(Some)
				.map_err(jsonrpsee::core::Error::ParseError)
		} else {
			Ok(None)
		}
	}
}
