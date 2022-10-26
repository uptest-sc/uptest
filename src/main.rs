<<<<<<< HEAD
// Uptest --chain X --wshost ws://host:port --pallet-test scheduler --sudo "seed goes here"

=======
use libuptest;
>>>>>>> bd4876f674d2491fc302c61eac7678ffe981aac8

use clap::ArgMatches;
mod cli;


fn main() {
    let matches: ArgMatches  = cli::gen_cli().get_matches();
}
