use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct RandomPlayer {
    config: Config,
    rng: SmallRng,
}

impl RandomPlayer {
    pub fn new(config: Config) -> RandomPlayer {
        RandomPlayer {
            config,
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn search(&mut self, s: &GroundState) -> Option<PureMove> {
        let (hop1zuo1_candidates, candidates) = s.get_candidates(self.config);
        let pure_move_1 = hop1zuo1_candidates.choose(&mut self.rng);
        let pure_move_2 = candidates.choose(&mut self.rng);
        pure_move_1.or(pure_move_2).cloned()
    }

    pub fn search_excited(
        &mut self,
        m: &InfAfterStep,
        s: &ExcitedState,
    ) -> Option<AfterHalfAcceptance> {
        let candidates = s.get_candidates(self.config);
        candidates.choose(&mut self.rng).copied()
    }

    pub fn search_hand_resolved(&mut self, s: &HandExists) -> Option<TymokOrTaxot> {
        [
            TymokOrTaxot::Tymok(s.if_tymok.clone()),
            TymokOrTaxot::Taxot(s.if_taxot.clone()),
        ]
        .choose(&mut self.rng)
        .cloned()
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
