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
        // select ws host
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
        .subcommand(
            Command::new("pallet-method-sub")
                .short_flag('s')
                .long_flag("method_subscribe")
                .about("Listen for new blocks and break when a certain pallet function is executed")
                .arg(arg!(<pallet_name> "name of pallet"))
                .arg(arg!(<pallet_method> "name of pallet function")),
        )
}
