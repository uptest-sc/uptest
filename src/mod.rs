pub mod cli;

use libuptest::jsonrpseeclient::subscription::HandleSubscription;
use libuptest::jsonrpseeclient::subscription::Subscribe;
use libuptest::jsonrpseeclient::{JsonrpseeClient, RpcParams, SubscriptionWrapper};
use libuptest::types::Header;

/// Subscribe and break on user defined event
pub fn watch_for_event(wshost: &str, pallet_name: &str, pallet_method: &str) -> bool {
    println!("");
}