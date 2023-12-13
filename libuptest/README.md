# Libuptest     

Substrate r*U*ntime u*P*grade *TEST*ing library

[![Crates.io LibUptest](https://img.shields.io/crates/v/libuptest.svg)](https://crates.io/crates/libuptest)
[![Docs.rs Libuptest](https://img.shields.io/docsrs/libuptest/0.1.1)](https://docs.rs/libuptest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)   


Initally funded by:  
![Polkadot Treasury](polkadot-treasury-logo.svg)

### Latest release:
0.1.4   

### Documentation:    
[https://uptest-sc.github.io/](https://uptest-sc.github.io/)   
[https://docs.rs/libuptest/0.1.1/libuptest/](https://docs.rs/libuptest/0.1.1/libuptest/)   



## Uptest 

Uptest aims to be an easy stand alone library for testing runtime upgrades before they are deployed.  
Use libuptest to execute extrensic test before and after the upgrade is pushed

### Contribute   
[Submit a github issue](https://github.com/uptest-sc/uptest/issues/new)


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


### Get all pallet's storagevalues and storage maps:   
```shell
cargo run -p uptest-examples --example get_pallet_storagemaps_storagevalues
```

### Default connection endpoints:    
The rpcclient comes with 5 different "default" chain endpoints:     
*  with_default_url -> ws://127.0.0.1:9944   
*  edgeware_default_url -> wss://edgeware.jelliedowl.net:443    
*  polkadot_default_url -> wss://polkadot-rpc-tn.dwellir.com:443   
*  kusama_default_url -> wss://kusama-rpc-tn.dwellir.com:443   
*  sora_default_url -> wss://ws.mof.sora.org:443   

