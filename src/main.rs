// Uptest --chain X --wshost ws://host:port --pallet-test scheduler --sudo "seed goes here"


//use crate::libuptest;

use clap::ArgMatches;
mod cli;

//use libuptest::ws_mod::get_remote_metadata_version;

fn main() {
    println!("uptest start");
    let matches: ArgMatches  = cli::gen_cli().get_matches();
    println!("Matches: {:?}", matches.subcommand_name())
}
