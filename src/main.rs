use cetkaik_core::absolute::Side;
use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use rand::prelude::*;
pub struct HandExists {
    pub if_tymok: GroundState,
    pub if_taxot: IfTaxot,
}

#[derive(Clone)]
pub enum TymokOrTaxot {
    Tymok(GroundState),
    Taxot(IfTaxot),
}

fn do_match() {
    let quiet = false;
    let config = Config::cerke_online_alpha();
    let mut rng = SmallRng::from_entropy();
    let mut state = initial_state().choose().0;
    loop {
        match state.whose_turn {
            Side::IASide => println!("IASide"),
            Side::ASide => println!("ASide"),
        }
        let pure_move = {
            let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
            let pure_move_1 = hop1zuo1_candidates.choose(&mut rng);
            let pure_move_2 = candidates.choose(&mut rng);
            pure_move_1.or(pure_move_2).cloned()
        };
        if pure_move.is_none() {
            break;
        }
        let pure_move = pure_move.unwrap();
        if !quiet {
            println!("Move: {:?}", pure_move);
        }
        let hnr_state = match pure_move {
            PureMove::NormalMove(m) => apply_normal_move(&state, m, config).unwrap().choose().0,
            PureMove::InfAfterStep(m) => {
                let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose().0;
                let aha_move = ext_state
                    .get_candidates(config)
                    .choose(&mut rng)
                    .copied()
                    .unwrap();
                if !quiet {
                    println!("Move(excited): {:?}", aha_move);
                }
                apply_after_half_acceptance(&ext_state, aha_move, config)
                    .unwrap()
                    .choose()
                    .0
            }
        };
        let resolved = resolve(&hnr_state, config);
        match &resolved {
            HandResolved::NeitherTymokNorTaxot(s) => state = s.clone(),
            HandResolved::HandExists { if_tymok, if_taxot } => {
                let he = HandExists {
                    if_tymok: if_tymok.clone(),
                    if_taxot: if_taxot.clone(),
                };
                match [
                    TymokOrTaxot::Tymok(he.if_tymok.clone()),
                    TymokOrTaxot::Taxot(he.if_taxot.clone()),
                ]
                .choose(&mut rng)
                .cloned()
                .unwrap()
                {
                    TymokOrTaxot::Tymok(s) => state = s,
                    TymokOrTaxot::Taxot(t) => {
                        if !quiet {
                            println!("Taxot!");
                        }
                        match t {
                            IfTaxot::NextSeason(ps) => state = ps.clone().choose().0,
                            IfTaxot::VictoriousSide(v) => {
                                println!("Won: {:?}", v);
                                break;
                            }
                        }
                    }
                }
            }
            HandResolved::GameEndsWithoutTymokTaxot(v) => {
                println!("Won: {:?}", v);
                break;
            }
        }
    }
}

fn main() {
    do_match();
}
