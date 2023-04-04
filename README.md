## Uptest 

Uptest aims to be an easy stand alone library for testing runtime upgrades before they are deployed.  
Use libuptest to execute extrensic test before and after the upgrade is pushed

#### substrate runtime *UP*grade *TEST*ing suit

With uptest you can: 
*  Easily query changes mades to types, storagemaps etc..  
*  Quickly get started with writing extrinsic tests with libuptest, let the library handle the execution logic before and after the runtime upgrade takes place   
*  Utilize a standalone rust library to build out your integration tests
*  Filter output based on pallet, to get a detailed change log    
*  Schedule tests to execute before and/or after a runtime upgrade takes place, developers does not need to spend any extra time, tracking the state and execution flow of the runtime upgrade with polkadot.js typescript

See future [usage_future.md](usage_future.md) for more details



### Got a feature you want us to add on the roadmap?   
[Submit a github issue](https://github.com/uptest-sc/uptest/issues/new)


### To be avaliable with cargo in the near future:  
https://crates.io/crates/uptest

### Check out future planned features here:   
https://github.com/users/uptest-sc/projects/1/views/1   


### Examples    

#### Get metadata version:   
```rust
use libuptest::jsonrpseeclient::JsonrpseeClient;
use libuptest::ws_mod::get_metadata_version;

println!("Connecting to Edgeware");
let dial_edg: JsonrpseeClient = JsonrpseeClient::edgeware_default_url().unwrap();//.unwrap();//.unwrap();
let edg_version: u8 = get_metadata_version(dial_edg).await?;
   
println!("Connected to chain: {:?} and got metadata version: {:?}", "Edgeware", edg_version);
```   
`cargo run -p uptest-examples --example metadata_version`   



## Runtime Migrations links:   
https://github.com/apopiak/substrate-migrations   
https://substrate-developer-hub.github.io/docs/en/knowledgebase/runtime/upgrades   
https://docs.substrate.io/reference/how-to-guides/parachains/runtime-upgrade/   
https://github.com/paritytech/substrate/issues?q=label%3AE1-runtimemigration%20   
