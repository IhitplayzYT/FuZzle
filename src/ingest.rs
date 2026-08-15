pub mod ingestor{
    use std::{collections::HashMap, panic::PanicHookInfo, path::Path};

use crate::{helper::Helper::{PLAYERS, STRATEGIES}, model::model::{Game, Player, Strategy}};


    pub fn read_file(path:&str) -> Result<String,std::io::Error>{
        Ok(std::fs::read_to_string(path)?)
    }

    pub fn parse_file(path:&str) -> Option<Game>{
        let file = read_file(path).unwrap();
        let mut players= None;
        let mut strategies = None;
        for i in file.split("\n\n"){
            if i.contains(PLAYERS){
                players = Some(parse_players(&i[i.find(PLAYERS).unwrap()+PLAYERS.len()..]));
            }else if i.contains(STRATEGIES){
                strategies = Some(parse_strategies(&i[i.find(STRATEGIES).unwrap()+STRATEGIES.len()..]));
            }   
        }

        if players.is_none() || strategies.is_none() || players.clone().unwrap().len() != strategies.unwrap().len(){
            return None;
        }

        let players = players.unwrap().into_iter().enumerate().map(|(idx,x)| (idx,Player::new(Some(x)))).collect::<Vec<(usize,Player)>>();
        let mut ret = Game::from_extracted(players);
        ret.game_name = Some(if path.contains("/"){path[path.rfind("/").unwrap()+1..].to_string()} else{path.to_string()});
        Some(ret)
    }


    pub fn parse_strategies(startegies: &str) -> HashMap<String,Vec<Strategy>>{
        let mut ret = HashMap::new();
        for i in startegies.trim().split("\n"){
            if let Some(idx) = i.find(":"){
                let (player,strat) = (i[..idx].trim().to_string(),Strategy::from(i[idx+1..].trim()));
                ret.entry(player).or_insert(vec![]).push(strat);
            }
        }
        ret
    }

    pub fn parse_players(players: &str) -> Vec<String>{
        players.trim().split("\n").map(|x| x.to_string()).collect::<Vec<String>>()
    }


  
}