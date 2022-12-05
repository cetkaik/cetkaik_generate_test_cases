use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct RandomPlayer {
    pub config: Config,
    pub rng: SmallRng,
}

impl RandomPlayer {
    pub fn new(config: Config) -> RandomPlayer {
        RandomPlayer {
            config,
            rng: SmallRng::from_entropy(),
        }
    }
}

pub struct HandExists {
    pub if_tymok: GroundState,
    pub if_taxot: IfTaxot,
}

#[derive(Clone)]
pub enum TymokOrTaxot {
    Tymok(GroundState),
    Taxot(IfTaxot),
}
