pub mod model{
    use crate::model::model::Strategy::{Deterministic, Fuzzy};

    
    #[derive(Debug,Clone,PartialEq,Copy)]
    pub enum Strategy{
        Deterministic(f64),
        Fuzzy(f64,f64,f64)
    }


use rand::Rng;

fn sample_triangular(min: f64, avg: f64, max: f64) -> f64 {
    assert!(min <= avg && avg <= max);
    if min == max {
        return min;
    }

    let mut rng = rand::rng();
    let u: f64 = rng.random();
    let p = (avg - min) / (max - min);

    if u < p {
        min + (u * (max - min) * (avg - min)).sqrt()
    } else {
        max - ((1.0 - u) * (max - min) * (max - avg)).sqrt()
    }
}


    impl Strategy{
        pub fn reward(&self) -> f64{
            match self{
                Self::Deterministic(x) => {*x},
                Self::Fuzzy(mn,avg,mx) => {sample_triangular(*mn, *avg, *mx)}
            }
        }

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
                            Fuzzy(mn,mn+(mx-mn)/2.0,mx)
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

        pub fn get_n_moves(&self) -> usize{
            self.players[0].1.stategies.len()
        }

        pub fn get_n_players(&self) -> usize{
            self.players.len()
        }


        pub fn get_strategies(&self,profiles: &Vec<usize>) -> Vec<Strategy>{
            self.players.iter().enumerate().map(|x| x.1.1.stategies[profiles[x.0]].1).collect::<Vec<Strategy>>()
        }

        pub fn build_strategy_str(&self,profiles: &Vec<usize>) -> String{
            self.players.iter().enumerate().map(|x| format!("{:?}",x.1.1.stategies[profiles[x.0]].1)).collect::<Vec<String>>().join(", ")
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

        pub fn compute_payoff(&self,profile:& Vec<usize>) -> Vec<f64>{
            let mut ret = Vec::with_capacity(profile.len());
            for (idx,v) in profile.iter().enumerate(){
                ret[idx] = self.players[idx].1.stategies[*v].1.reward()
            }
            ret
        }


        pub fn is_nash(&self,mut profile: Vec<usize>) -> bool{
            let payoff = self.compute_payoff(&profile);
            for (idx,_) in &self.players{
                let strategy = profile[*idx];
                let reward = payoff[*idx];
                for (i,_) in &self.players[*idx].1.stategies{
                    if *i == strategy{
                        continue
                    }
                    profile[*idx] = *i;
                    if self.compute_payoff(&profile)[*idx] > reward{
                        return false;
                    }
                    profile[*idx] = strategy;
                }
            }
           true 
        }


    }




}