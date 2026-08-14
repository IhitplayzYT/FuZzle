use std::process::exit;

use crate::helper::Helper::CLI;

mod helper;
mod model;
mod ingest;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    if clargs.path.is_none(){
        eprintln!("Path to a yaml file containing the game specs is required");
        exit(-1);
    }



}
