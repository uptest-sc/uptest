// Uptest --chain X --wshost ws://host:port --pallet-test scheduler --sudo "seed goes here"


use clap::ArgMatches;
mod cli;


fn main() {
    let matches: ArgMatches  = cli::gen_cli().get_matches();
}
