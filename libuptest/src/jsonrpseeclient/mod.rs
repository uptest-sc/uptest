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
/*
Modified by:
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use crate::error::Error; // {Error, Request, Result, RpcParams, Subscribe};
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

pub use rpcstuff::RpcParams;
pub use subscription::SubscriptionWrapper;

pub mod rpcstuff;
pub mod subscription;

pub use subscription::Result;

#[derive(Clone)]
pub struct JsonrpseeClient {
    inner: Arc<Client>,
}

impl JsonrpseeClient {
    pub fn new(url: &str) -> Result<Self> {
        block_on(Self::async_new(url))
    }

    /// connect to ws://127.0.0.1:9944  
    pub fn with_default_url() -> Result<Self> {
        Self::new("ws://127.0.0.1:9944")
    }

    /// connect wss://edgeware.jelliedowl.net:443
    pub fn edgeware_default_url() -> Result<Self> {
        Self::new("wss://edgeware.jelliedowl.net:443")
    }

    /// connect to wss://polkadot-rpc-tn.dwellir.com:443
    pub fn polkadot_default_url() -> Result<Self> {
        Self::new("wss://polkadot-rpc-tn.dwellir.com:443")
    }

    /// connect to wss://kusama-rpc-tn.dwellir.com:443
    pub fn kusama_default_url() -> Result<Self> {
        Self::new("wss://kusama-rpc-tn.dwellir.com:443")
    }

    /// connect to wss://ws.mof.sora.org:443  
    pub fn sora_default_url() -> Result<Self> {
        Self::new("wss://ws.mof.sora.org:443")
    }

    pub fn get_metadata_version() -> Result<u32> {
        Ok(1u32)
    }

    async fn async_new(url: &str) -> Result<Self> {
        // todo change errors
        let uri: Uri = url.parse().map_err(|_e| Error::ConnectionClosed)?; //no need to map the error if its generic
        let (tx, rx) = WsTransportClientBuilder::default()
            .build(uri)
            .await
            .map_err(|_e| Error::ConnectionClosed)?;
        let client = ClientBuilder::default()
            .max_notifs_per_subscription(4096)
            .build_with_tokio(tx, rx);
        Ok(Self {
            inner: Arc::new(client),
        })
    }
}

#[maybe_async::async_impl(?Send)]
impl Request for JsonrpseeClient {
    async fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
        // Support async: #278
        let future = self.inner.request(method, RpcParamsWrapper(params)).await;
        future.map_err(|_e| Error::ConnectionClosed)
    }
}
#[maybe_async::sync_impl]
impl Request for JsonrpseeClient {
    fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
        block_on(self.inner.request(method, RpcParamsWrapper(params)))
            .map_err(|e| Error::ConnectionClosed)
    }
}

/*

#[maybe_async::async_impl(?Send)]
impl Request for JsonrpseeClient {
    async fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
        // Support async: #278
        let future = self.inner.request(method, RpcParamsWrapper(params)).await;
        future.map_err(|e| Error::Client(Box::new(e)))
    }
}
#[maybe_async::sync_impl]
impl Request for JsonrpseeClient {
    fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R> {
        block_on(self.inner.request(method, RpcParamsWrapper(params)))
            .map_err(|e| Error::Client(Box::new(e)))
    }
}
*/

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
            .map_err(|_e| Error::ConnectionSubscriptionProblem) // todo solve better error handling here
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
