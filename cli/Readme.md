# Uptest-cli   

Command line client for Libuptest.

Substrate r*U*ntime u*P*grade *TEST*ing library

[![Crates.io uptest-cli](https://img.shields.io/crates/v/uptest-cli.svg)](https://crates.io/crates/uptest-cli)
[![Docs.rs Libuptest](https://img.shields.io/docsrs/uptest-cli/0.1.1)](https://docs.rs/uptest-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)   

Funded by:  
![Polkadot Treasury](polkadot-treasury-logo.svg)


### Documentation:    
[https://uptest-sc.github.io/](https://uptest-sc.github.io/)   
[https://docs.rs/uptest-cli/0.1.1/uptest-cli/](https://docs.rs/uptest-cli/0.1.1/uptest-cli/)   

### Help  
```
$ ./target/release/uptest-cli --help
Uptest command line tool
substrate runtime UPgrade TESTing suit

Usage: uptest-cli <COMMAND>

Commands:
  pallet-method-sub, -s, --method_subscribe
          Listen for new blocks and break when a certain pallet function is executed
  storage-changes, -c, --storage_changes
          Displays the changes made to storage maps and storage values after a runtime upgrade, similar to pallets-storage(-p) but subscribes to chain
          Subscribes to chain with a 150 block limit > wait for sudo runtime upgrade tx > get's new storage types > compares old types to new storage types and displays a diff/changelog
                      
  chain-info, -i, --chain-info
          Displays meta information about the chain
  pallets-storage, -p, --pallets_storage
          Displays all storage maps and storage values for all pallets
  pallet-storage, -d, --pallet_storage
          Displays storage maps and storage values for a single pallet
  block-watch, -w, --block_watch
          Subscribe to a chain and display the events triggered in the latest blocks
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

