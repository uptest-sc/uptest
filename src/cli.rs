// cli
use clap::{arg, Command};

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
        .subcommand(
            Command::new("wshost")
                .short_flag('w')
                .long_flag("wshost")
                .about("Ws socket host to use")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg(arg!(--sudokey "use sudo to upgrade chain"))
                .arg_required_else_help(true), //require a ws host
        )
/*  todo
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
        // read local wasm file and submit runtime upgrade
        .subcommand(
            Command::new("submit-wasm")
            .short_flag('u')
            .long_flag("submit_upgrade")
            .help_template("uptest -u /tmp/runtime_blob.wasm")
            .about("Submit runtime upgrade wasm file")
            .arg(arg!(<wasm_filepath> "file path of wasm file")),
        )
}
