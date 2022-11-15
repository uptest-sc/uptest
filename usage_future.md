### Command line flags     
name, optional, description            
--ws, optional, ws socket host to connect to             
--rpc, optional user must specify either rpc or ws, rpc host to connect to            
--tests(short -t), optional, location of test file        
--chain(short -c), optional, manually specify the chain        
--metadata-version, optional, use a specific metadata version to connect to the chain, if no version is given uptest will try to auto guess the version    
--wasm-file(short -w), ,compiled node runtime/wasm file to submit as a runtime upgrade             
--sudo-key optional, provide a sudo key, push the runtime upgrade with sudo/enables sudo transactions
--pallet-diff, optional, provide a list of added and removed pallets after runtime upgrade
--fork-live, optional, fork a live chain, outputs chainspec file
--fork-key, optional, replace sudo and validator keys with the user-provided key
--insert-val-key, optional, submit validator key to node
--verbose(short -v), optional, enable verbose output        
--account-key, optional, use a custom key for submitting transactions  
--function-update-check, detect changes is public pallet functions
--git, optional, clone a git repo and generate template tests files for all pallets in construct_runtime   
--skip-gen-pallet, optional, do not generate pallet tests files for these. Separated by commas
--pallet-version, optional, specify a specific pallet version to check if it changed during the runtime upgrade                          
--pallet-prefix, optional, specify a specific prefix to check if it changed during the runtime upgrade, for example: "py/trsry"
--pallet-name, optional, specify the pallet you want to interact to 
--pallet-info, optional, query a live chain for details for a specific pallet   
--pallet-storage-diff, optional, display changes made to a pallets storage
--balance-diff, optional, Display accounts with update account balance    
--connect-at-block, optional, specify what block you want to use when interacting with the chain, if not set Uptest will connect at the latest finalized block            
--migration-search, optional, Search for migration code in Pallet. 
--migration-branch, optional, specify the git branch to use with --migration-search 
--migration-git, optional, use a custom github repo for searching with --migration-search, defaults to: https://github.com/paritytech/substrate/


## Usage examples:  
```
$ uptest --help
```
We have choice to take the "Polkadot.js on steroids" approach where we want to give the developer a tool to identify and test needed migrations.

## Fork a chain with uptest:
```  
$ uptest --fork-live --fork-key <USERKEY> --rpc http://rpc.mychain.local > newchain
```    

## Start the forked chain:  
```
$ ./chainbinary --chain newchain --ws-port 1337
```
Submit validator key

```
$ uptest --ws ws://127.0.0.1:1337 --insert-val-key "<USERKEY>"
```
Restart the chain with --validator:
```
$ ./chainbinary --chain newchain --ws-port 1337 --validator
```


## Pallet version   
Detect if a pallet has changed its version during the runtime upgrade
```bash
$ uptest --ws ws://127.0.0.1:1337 --sudo <KEY> --pallet-version  --pallet-name Scheduler  -w runtime.wasm
[debug] Connected to chain X with spec_version: Z
[debug] Current pallet Scheduler version: 3
[debug] Pushing runtime upgrade with sudo
[debug] Runtime update detected in block #1112
[debug] After upgrade pallet Scheduler version: 4
[debug] Pallet Balance has upgraded version from 3 to 4
```

Note: 
Add ` --pallet-name Balance ` to check for a specific pallet otherwise, Uptest will check all pallet versions it can find

## Pallet migration search   
Now that we know that a new version of pallet "Scheduler" has been rolled out in the runtime upgrade, we want to see if the new version of the pallet comes with built in migrations:
```
$ uptest --migration-search Scheduler --migration-branch "polkadot-v0.9.32"
[debug] Searching for pallet Scheduler in https://github.com/paritytech/substrate/ branch polkadot-v0.9.32  
[debug] Found Pallet Scheduler Version: 4.0.0-dev
[debug] Pallet Scheduler contains migration.rs
Uptest found migrations v3 to v4 in Pallet Scheduler
```


## Pallet prefix  
Check if a pallet has changed its prefix:
```bash 
$ uptest --ws ws://127.0.0.1:1337 --pallet-prefix "py/trsry" -w runtime.wasm          
[debug] Connected to chain X with spec_version: A          
[debug] Found pallet prefix "py/trsy"          
[debug] Pushing runtime upgrade
[debug] runtime upgrade detected in block #1111
[debug] Prefix not found in new spec_version B
Pallet prefix has been removed in spec_version B
```

## Find added and removed pallets    
Display a change log of removed and added pallets   
```
$ uptest --ws  ws://127.0.0.1:1337  -w runtime.wasm --pallet-diff
[debug]  build a list of current pallets
[debug] push runtime upgrade  
[debug] Comparing old pallet list to the new one
+ Pallet added: MultisigNew
+ Pallet added: BalancesNew
- Pallet removed: Balances
- Pallet removed: Multisig
```

## Check for upgrades/changes in a storage map

```rust
use libuptest::QueryTypeSMap; // query a storage map and return storage map type 
...

#[after_upgrade]
fn upgraded_chain_check() -> bool {
// check that if storagemap TEST has changed in pallet_test
let old_storage_type = get_pre_upgrade_storage_map_type(); // Get to storage map we saved before the runtime upgrade took place
let new_storage_type = QueryTypeSMap("pallet_test", "TEST"); // query the storage map after upgrade
println!("Checking if storage map TEST has changed");
assert_ne!(new_storage_type, old_storage_type); // Verify that the storage map has changed
println!("Storage map TEST has changed");
}


```

## Query for pallet storage changes made after a runtime upgrade   
```
$ uptest --ws  ws://127.0.0.1:1337  -w runtime.wasm  --pallet-storage-diff --pallet-name fluff 
[debug - preupgrade] Found Pallet "fluff" with the following storagemaps: 
Astorage(Address, u64)
Bstorage(Address, Address, u32)
Cstorage(Address, Address, Address)
[debug] Pushing runtime upgrade  
[debug] Storage "Bstorage" changed from "Bstorage(Address, Address, u32)" to "Bstorage(Address, u32)" 
```


## Check for changes in pallet functions after a runtime upgrade    

```
uptest --ws  ws://127.0.0.1:1337  -w runtime.wasm --function-update-check 
[debug] Pushing runtime upgrade
[debug] + Pallet "Template" changed function from "do_something(origin, u32)" to "do_something(origin, u32, u64)" 
[debug] + Pallet "Balances" changed function t"ransfer_all(origin, AccountIdLookupOF, bool)" to "transfer_all(origin, AccountIdLookupOF, bool, u64)"  
```
Note:
Check individual pallets by specifying the --pallet-name flag.


# Write test with libuptest macro   


## example pallet tests
```rust 
use libuptest::macros::{pre_upgrade, after_upgrade, AccountCreate};
use libuptest::{PalletSubmit, Querychain};

// execute test before runtime upgrade
#[pre_upgrade]
fn first_tests() -> bool {
let origin = AccountCreate("<SEEDPHRASE GOES HERE>");
PalletSubmit(PalletName, PalletFunction, PalletInput);
}
// execute the test function after the runtime upgrade
#[after_upgrade]
fn second_tests() -> bool {
...
}

```


## Automatically generate tests   
Generate template test files with uptest based on the find public function and storage types uptest find in each pallet.  

```
$ uptest --git https://github.com/substrate-developer-hub/substrate-node-template/ --skip-gen-pallet System, Aura, Grandpa
Generating tests
Uptest has now generated tests for pallet RandomnessCollectiveFlip, Balances, Sudo, template tests saved in the test folder
$ ls tests/
pallet_randomness_collective_flip.rs 
pallet_balances.rs
pallet_transaction_payment.rs
pallet_sudo.rs
pallet_template.rs
```
```
$ cat tests/pallet_template.rs
```
```rust
// File automatically generated with uptest  
// functions found: do_something
// storage found: Something

use libuptest::{rng, PalletSubmit, AccountCreate, QueryTypeSValue};
use libuptest::macros::{pre_upgrade, after_upgrade};

// public call pub fn do_something(origin: OriginFor<T>, something: u32)
#[pre_upgrade]
fn first_do_something() -> bool {
let input_u32 = rng::gen_u32(); // generate a random u32   
let origin = AccountCreate("USER PROVIDED KEY");
PalletSubmit("TemplateModule", "do_something", input_u32);

}

// storage maps found in the pallet
#[pre_upgrade]
fn first_storage_test() -> bool {
let storage_type = QueryTypeSValue("pallet_template", "Something"); 
}

#[after_upgrade]
fn second_do_something() -> bool {
...
}

```

Uptest will generate a template for the public pallet functions and the storage maps it found for each pallet

## Display changed balances after runtime upgrade   
If we have updated an account's balance with a runtime upgrade, we can simply display all accounts that have changed balances after the runtime upgrade.
```
$ uptest --ws ws://127.0.0.1:1337 -w runtime.wasm --balance-diff       
[debug] indexing balances
[debug] pushing runtime upgrade
+ Balance increased for Address:   5FscQ6DX8rwMwhAoeRNitpoDeUWq3QGNxsvJ5L7VoTiURRZk  changed from 50 to 200          
+ Balance increased for Address:   5GuzJsHXzkSStfToPcAuWZfpKXmrpfPMKyiySjNpjRNkqbG8  changed from 100 to 200    
-  Balance decreased for Address:  5DykY2Yac5XjFfMmHFiJVACCoDvoVedvK45A2ryX4msXZP4X  changed from 100 to 50
```


Uptest default execution flow:   
```
<Connect to chain>
<handles chains metadata>
<first line of test>
<submit runtime upgrade>
<probe/wait for runtime upgrade>
<second line of tests>   
<Verdict>
```
