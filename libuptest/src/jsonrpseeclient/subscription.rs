/*
based on:
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

extern crate alloc;
use crate::error::Error; //rpc::{, HandleSubscription, Result};
use crate::jsonrpseeclient::rpcstuff::RpcParams;
use alloc::string::{String, ToString};
use futures::executor::block_on;
use jsonrpsee::core::client::Subscription;
use serde::de::DeserializeOwned;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct SubscriptionWrapper<Notification> {
    inner: Subscription<Notification>,
}

impl<Notification: DeserializeOwned> HandleSubscription<Notification>
    for SubscriptionWrapper<Notification>
{
    fn next(&mut self) -> Option<Result<Notification>> {
        block_on(self.inner.next()).map(|result| result.map_err(|e| Error::Client(Box::new(e))))
    }

    fn unsubscribe(self) -> Result<()> {
        block_on(self.inner.unsubscribe()).map_err(|_e| Error::Unsubscribefail) // todo solve better error handling here
    }
}

impl<Notification> From<Subscription<Notification>> for SubscriptionWrapper<Notification> {
    fn from(inner: Subscription<Notification>) -> Self {
        Self { inner }
    }
}

/// Trait to be implemented by the ws-client for sending rpc requests and extrinsic.
#[maybe_async::maybe_async(?Send)]
pub trait Request {
    /// Sends a RPC request to the substrate node and returns the answer as string.
    async fn request<R: DeserializeOwned>(&self, method: &str, params: RpcParams) -> Result<R>;
}

/// Trait to be implemented by the ws-client for subscribing to the substrate node.
pub trait Subscribe {
    type Subscription<Notification>: HandleSubscription<Notification>
    where
        Notification: DeserializeOwned;

    fn subscribe<Notification: DeserializeOwned>(
        &self,
        sub: &str,
        params: RpcParams,
        unsub: &str,
    ) -> Result<Self::Subscription<Notification>>;
}

/// Trait to use the full functionality of jsonrpseee Subscription type
/// without actually enforcing it.
pub trait HandleSubscription<Notification: DeserializeOwned> {
    /// Returns the next notification from the stream.
    /// This may return `None` if the subscription has been terminated,
    /// which may happen if the channel becomes full or is dropped.
    ///
    /// **Note:** This has an identical signature to the [`StreamExt::next`]
    /// method (and delegates to that). Import [`StreamExt`] if you'd like
    /// access to other stream combinator methods.
    fn next(&mut self) -> Option<Result<Notification>>;

    /// Unsubscribe and consume the subscription.
    fn unsubscribe(self) -> Result<()>;
}

pub fn to_json_req(method: &str, params: RpcParams) -> Result<String> {
    Ok(serde_json::json!({
        "method": method,
        "params": params.to_json_value()?,
        "jsonrpc": "2.0",
        "id": "1",
    })
    .to_string())
}
