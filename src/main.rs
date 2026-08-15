use std::process::exit;

use crate::{helper::Helper::CLI, ingest::ingestor::parse_file};

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

    let game = parse_file(&clargs.path.clone().unwrap());
    if let Some(x) = game{
        println!("Game: {} consists of {} players",x.game_name.unwrap(),x.players.len());
                    


    }else{
        panic!("Failed in Parsing a game from {}",clargs.path.unwrap())
    }
}
