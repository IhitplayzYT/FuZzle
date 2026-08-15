pub mod model{
    use crate::model::model::Strategy::{Deterministic, Fuzzy};

    
    #[derive(Debug,Clone,PartialEq,Eq,Hash,Copy)]
    pub enum Strategy{
        Deterministic(usize),
        Fuzzy(usize,usize,usize)
    }

    impl From<&str> for Strategy{
        fn from(value: &str) -> Self {
            if !value.contains("("){
                return Strategy::Deterministic(value.parse().expect("The playoffs should be floats"));
            }else{
                let fields = value[value.find("(").unwrap()+1..value.find(")").unwrap()].split(",").collect::<Vec<&str>>();
                    return match fields.len(){
                        1 => {Deterministic(fields[0].parse().expect("The playoffs should be floats"))},
                        2 => {
                            let (mn,mx) = (fields[0].parse().expect("The playoffs should be floats"),fields[1].parse().expect("The playoffs should be floats"));
                            Fuzzy(mn,mn+(mx-mn)/2,mx)
                        },
                        3 => {Fuzzy(fields[0].parse().expect("The playoffs should be floats"),fields[1].parse().expect("The playoffs should be floats"),fields[2].parse().expect("The playoffs should be floats")) },
                        _ => {panic!("Invalid Strategies field");}
                    };

            }
        }

    }


    #[derive(Debug,Clone)]
    pub struct Player{
        pub name: Option<String>,
        pub stategies: Vec<(usize,Strategy)>
    }

    impl Player{
        pub fn new(name: Option<String>) -> Self{
            Self { name, stategies: vec![]}
        }

    }


    #[derive(Debug,Clone)]
    pub struct Game{
        pub players: Vec<(usize,Player)>,
        pub strat_profile: Vec<Vec<usize>>,
        pub game_name: Option<String>
    }

    impl Game{
        pub fn new(n_players: usize,n_strat:usize,name: Option<String>) -> Self{    
            Self { players: Vec::with_capacity(n_players), strat_profile: Vec::with_capacity(n_strat.pow(n_players as u32)),game_name:name}
        }

        pub fn get_strat(&self,i:usize,j:usize) -> Strategy{
            self.players[j].1.stategies[self.strat_profile[i][j]].1
        }


      pub fn from_extracted(players: Vec<(usize, Player)>) -> Self {
        let mut strat_profile: Vec<Vec<usize>> = vec![Vec::new()];
        for (_, player) in &players {
            let mut update = Vec::new();
            for profile in &strat_profile {
                for (strategy_idx, _) in &player.stategies {
                    let mut n_profile = profile.clone();
                    n_profile.push(*strategy_idx);
                    update.push(n_profile);
                }
            }
            strat_profile = update;
        }
        Self {players,strat_profile,game_name:None}
    }
    }




}