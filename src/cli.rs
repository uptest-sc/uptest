/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/
// cli
use clap::{arg, Command};

fn get_git_hash() -> String {
	let output = std::process::Command::new("git")
		.args(&["rev-parse", "--verify", "HEAD"])
		.output().unwrap();
	let git_hash = String::from_utf8(output.stdout).unwrap_or_default();
	git_hash.trim().to_string()
}

pub fn gen_cli() -> Command {

     Command::new("uptest")
        .about("substrate runtime UPgrade TESTing suit")
        .version(env!("CARGO_PKG_VERSION")) // read from Cargo.tomlss
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author(env!("CARGO_PKG_AUTHORS"))
        // ws subcommand
        //
        // select ws host, lets make this required flag so we can use this with all 

/*  todo

        .subcommand(
            Command::new("wshost")
                .short_flag('w')
                .long_flag("wshost")
                .about("Ws socket host to use")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg(arg!(--sudokey "use sudo to upgrade chain"))
                .arg_required_else_help(true), //require a ws host
        )


        // select chain
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("chain")
                .short_flag('c')
                .long_flag("chain")
                .about("Select chain")
                .arg(arg!(<CHAIN> "chain to connect to"))
                .arg_required_else_help(true),
        )
        // pallet tests
        .subcommand(
            Command::new("pallet-test")
                .short_flag('p')
                .long_flag("test")
                .about("pallet test")
                .arg(arg!(<testfile> "pallet test file to use")),
        )
        // generate tests
        .subcommand(
            Command::new("gen-test")
                .short_flag('g')
                .long_flag("gentest")
                .about("generate tests for pallets")
                .arg(arg!(<directory> "directory to use")),
        )
 */
        // subscribe to chain and find a certain event that got triggerd
        .subcommand(
            Command::new("pallet-method-sub")
                .short_flag('s')
                .long_flag("method_subscribe")
                .help_template("Usage example: uptest -s Balance transfer \r\nuptest -s pallet_name pallet_method")
                .about("Listen for new blocks and break when a certain pallet function is executed")
                .arg(arg!(<pallet_name> "name of pallet"))
                .arg(arg!(<pallet_method> "name of pallet function")),
        )
        .subcommand(
            Command::new("storage-changes")
            .short_flag('c')
            .long_flag("storage_changes")
            .about("Displays the changes made to storage maps and storage values after a runtime upgrade, similar to pallets-storage(-p) but subscribes to chain")
            .help_template("Usage example: uptest -c -w ws://127.0.0.1:9944")
            .arg(arg!(<ws> "ws endpoint of the chain to connect")),
        )
        // todo filter by custom pallet
        .subcommand(
            Command::new("pallets-storage")
            .short_flag('p')
            .long_flag("pallets_storage")
            .about("Displays all storage maps and storage values for all pallets")
            .help_template("Usage example: uptest -p ws://127.0.0.1:9944")
            .arg(arg!(<ws> "ws endpoint of the chain to connect")),
        )
        // Get all storage values and maps for a single pallet
        .subcommand(
            Command::new("pallet-storage")
            .short_flag('d')
            .long_flag("pallet_storage")
            .about("Displays storage maps and storage values for a single pallet")
            .help_template("Usage example: uptest -d ws://127.0.0.1:9944 pallet_name \r\n uptest -d ws://127.0.0.1:9944 TemplateModule")
            .arg(arg!(<ws> "ws endpoint of the chain to connect"))
            .arg(arg!(<pallet_name> "pallet name")),
        )
        .subcommand(
            Command::new("block-watch")
            .short_flag('w')
            .long_flag("block_watch")
            .about("Subscribe to a chain and display the events triggered in the latest blocks")
            .help_template("
            Usage example: uptest -w wshost blockamount
            \r\n Connect to polkadot and view the latest 40 blocks: uptest -w wss://polkadot-rpc-tn.dwellir.com:443 40 
            \r\n Latest 50 blocks from the locally running substrate node: ./target/release/uptest -w ws://127.0.0.1:9944 50")
            .arg(arg!(<ws> "ws endpoint of the chain to connect"))
            .arg(arg!(<block_limit> "amount of blocks of latest blocks to subscribe to").required(true)),
        )
        // subscribe to the chain, display the latest blocks and events triggered in those blocks
        

        // TODO: read local wasm file and submit runtime upgrade
        .subcommand(
            Command::new("submit-wasm")
            .short_flag('u')
            .long_flag("submit_upgrade")
            .help_template("uptest -u /tmp/runtime_blob.wasm")
            .about("Submit runtime upgrade wasm file")
            .arg(arg!(<wasm_filepath> "file path of wasm file")),
        )
}
